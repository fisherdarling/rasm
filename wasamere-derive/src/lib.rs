extern crate proc_macro;

use crate::proc_macro::TokenStream;

use std::convert::TryInto;

use quote::{quote, ToTokens};
use syn::{
    parse2, parse_macro_input, AttrStyle, Attribute, Data, DataEnum, DataStruct, DeriveInput,
    Fields, FieldsNamed, FieldsUnnamed, Generics, Ident, Lit, LitInt, Variant,
};

use proc_macro2::TokenTree;

#[proc_macro_derive(Parse)]
pub fn parse_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;
    let generics = input.generics;

    match input.data {
        Data::Struct(data) => {
            let expanded = gen_struct_impl(data, generics, name);

            TokenStream::from(expanded)
        }
        Data::Enum(data) => {
            let expanded = gen_enum_impl(name, &input.attrs, generics, data);

            TokenStream::from(expanded)
        }
        _ => panic!("Input.Data Panic"),
    }
}

fn gen_struct_impl(data: DataStruct, generics: Generics, name: &Ident) -> proc_macro2::TokenStream {
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();;

    let fields = data.fields;

    let fields_do_parse = match fields {
        Fields::Named(ref fields) => gen_named_fields(fields),
        Fields::Unnamed(ref fields) => gen_unnamed_fields(fields),
        Fields::Unit => quote! { Ok((input, Self)) },
        _ => panic!("Fields Unnamed"),
    };

    let expanded = quote! {
        impl #impl_generics crate::parser::Parse for #name #ty_generics #where_clause {
            fn parse(input: &[u8]) -> IResult<&[u8], Self> {
                let res = #fields_do_parse;

                res
            }
        }
    };

    expanded
}

fn gen_enum_impl(
    name: &Ident,
    attrs: &[Attribute],
    generics: Generics,
    data: DataEnum,
) -> proc_macro2::TokenStream {
    // TODO: Handle Prefix Tagging

    let variant_parsers: Vec<_> = data
        .variants
        .iter()
        .map(|v| gen_variant_block(name, v))
        .collect();

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();;

    let expanded = quote! {
        impl #impl_generics crate::parser::Parse for #name #ty_generics #where_clause {
            fn parse(input: &[u8]) -> crate::parser::PResult<Self> {
                // use log::debug;

                // debug!("Parsing {}", stringify!(#name));

                let (input, value) = nom::do_parse!(input,
                    val: switch!(le_u8,
                        #(#variant_parsers)|*
                    ) >>
                    (val)
                )?;

                // debug!("Parsed {}: {:?}", stringify!(#name), value);

                Ok((input, value))
            }
        }
    };

    // println!("{}", expanded);

    expanded
}

fn gen_variant_block(enum_name: &Ident, variant: &Variant) -> proc_macro2::TokenStream {
    let attr = &variant.attrs[0];

    let variant_name = &variant.ident;

    let path = &attr.path;
    assert!(path.leading_colon.is_none());

    let kind = path.segments.iter().next().unwrap();

    let expanded = match kind.ident.to_string().as_str() {
        "byte" => gen_byte_block(enum_name, variant_name, &variant.fields, attr.tts.clone()),
        other => {
            println!("{}", other);
            panic!("Attr Path Block")
        }
    };

    expanded
}

fn gen_byte_block(
    enum_name: &Ident,
    variant_name: &Ident,
    fields: &Fields,
    stream: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    let lit: u8 = match stream.into_iter().next().unwrap() {
        TokenTree::Group(g) => parse2::<LitInt>(g.stream())
            .unwrap()
            .value()
            .try_into()
            .unwrap(),
        _ => panic!("Token Tree Panic"),
    };

    let resultant = match fields {
        Fields::Unit => {
            quote! {
                value!(#enum_name::#variant_name)
            }
        }
        Fields::Unnamed(fields) => {
            let field_tokens = gen_unnamed_fields(&fields);

            quote!(call!(
                |mut input: &[u8]| {
                    // let mut input = input;

                    Ok((input, #enum_name::#variant_name(#field_tokens)))
                }
            ))
        }
        Fields::Named(fields) => {
            let field_tokens = gen_named_fields(&fields);

            quote!(call!(
                |mut input: &[u8]| {
                    // let mut input = input;

                    Ok((input, #enum_name::#variant_name { #field_tokens }))
                }
            ))
        }

        _ => panic!("Field Panic Enum"),
    };

    let match_arm = quote! {
        #lit => #resultant
    };

    match_arm
}

fn gen_named_fields(fields: &FieldsNamed) -> proc_macro2::TokenStream {
    // let gen = Vec::new();

    let idents: Vec<_> = fields
        .named
        .iter()
        .map(|f| f.ident.clone().unwrap())
        .collect();
    let idents2 = idents.clone();

    let types: Vec<_> = fields.named.iter().map(|f| f.ty.clone()).collect();

    let expanded = quote! {
        nom::do_parse!(input,
            #(
                #idents: call!(<#types>::parse) >>
            )*
            (Self { #(#idents2),* } )
        )
    };

    expanded
}

fn gen_unnamed_fields(fields: &FieldsUnnamed) -> proc_macro2::TokenStream {
    let idents: Vec<_> = fields
        .unnamed
        .iter()
        .enumerate()
        .map(|(i, f)| {
            let name = format!("field_{}", i);
            let ident = Ident::new(&name, proc_macro2::Span::call_site());
            ident
        })
        .collect();
    let idents2 = idents.clone();

    let types: Vec<_> = fields.unnamed.iter().map(|f| f.ty.clone()).collect();

    let expanded = quote! {
        nom::do_parse!(input,
            #(
                #idents: call!(<#types>::parse) >>
            )*
            (Self ( #(#idents2),* ) )
        )
    };

    expanded
}