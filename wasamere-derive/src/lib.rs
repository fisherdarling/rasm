extern crate proc_macro;

use crate::proc_macro::TokenStream;

use std::convert::TryInto;

use quote::{quote, ToTokens};
use syn::{
    parse2, parse_macro_input, AttrStyle, Attribute, Data, DataEnum, DataStruct, DeriveInput,
    Field, Fields, FieldsNamed, FieldsUnnamed, Generics, Ident, Lit, LitInt, Meta, MetaList,
    NestedMeta, Variant,
};

use proc_macro2::TokenTree;

#[proc_macro_derive(Parse)]
pub fn parse_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;
    // let attrs = &input.attrs;
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
        Fields::Named(ref fields) => gen_named_fields(name, fields),
        Fields::Unnamed(ref fields) => gen_unnamed_fields(name, fields),
        Fields::Unit => quote! { Ok((input, Self)) },
        _ => panic!("Fields Unnamed"),
    };

    let expanded = quote! {
        impl #impl_generics crate::parser::Parse for #name #ty_generics #where_clause {
            fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {
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
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let meta: Meta = attrs[0]
        .parse_meta()
        .expect("Unable to parse attribute metadata");

    let list = match meta {
        Meta::List(list) => list,
        _ => unimplemented!("Enum impl only supports MetaList"),
    };

    let kind = list.ident.to_string();

    println!("Enum Attr Kind: {}", kind);

    let switch_parser = get_func_ident(&list);

    println!("Switch Parser: {}", switch_parser.to_string());

    // let name_variants: Vec<Ident> = data
    //     .variants
    //     .iter()
    //     .map(|v| {
    //         let new_name = format!("{}::{}", name.to_string(), v.ident.to_string());
    //         Ident::new(&new_name, proc_macro2::Span::call_site())
    //     })
    //     .collect();

    let match_arms: Vec<_> = data.variants.iter().map(get_match_arm).collect();

    let variant_parsers: Vec<_> = data
        .variants
        .iter()
        .map(|variant| match variant.fields {
            Fields::Named(ref named) => gen_enum_named_fields(name, &variant, named),
            Fields::Unnamed(ref unnamed) => gen_enum_unnamed_fields(name, &variant, unnamed),
            Fields::Unit => {
                println!("Unit Variant Field");
                let var_ident = &variant.ident;

                quote! {
                    value!(#name::#var_ident)
                }
            }
        })
        .collect();

    let expanded = quote! {
        impl #impl_generics crate::parser::Parse for #name #ty_generics #where_clause {
            fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {
                do_parse!(input,
                val: switch!(#switch_parser,
                        #(#match_arms => #variant_parsers)|*
                    ) >>
                    (val)
                )
            }
        }
    };

    println!("{}", expanded);

    expanded
}

fn get_match_arm(variant: &Variant) -> proc_macro2::TokenStream {
    let attr = &variant
        .attrs
        .get(0)
        .expect("Expected attribute for match arm.")
        .clone();
    let meta = attr
        .parse_meta()
        .expect("Unable to parse attribute metadata in match arm.");

    let list = match meta {
        Meta::List(list) => list,
        _ => unimplemented!("Get match arm => unknown metadata."),
    };

    let kind = list.ident.to_string();
    assert_eq!(&kind, "byte");

    let (expanded, len) = get_int_literals(&list);

    expanded
}

fn gen_named_fields(name: &Ident, fields: &FieldsNamed) -> proc_macro2::TokenStream {
    // let gen = Vec::new();

    let idents: Vec<_> = fields
        .named
        .iter()
        .map(|f| f.ident.clone().unwrap())
        .collect();
    let idents2 = idents.clone();

    // let types: Vec<_> = fields.named.iter().map(|f| f.ty.clone()).collect();
    let parsers: Vec<_> = fields.named.iter().map(gen_field_parser).collect();

    let expanded = quote! {
        nom::do_parse!(input,
            #(
                #idents: #parsers >>
            )*
            (#name { #(#idents2),* } )
        )
    };

    expanded
}

fn gen_enum_named_fields(
    name: &Ident,
    variant: &Variant,
    fields: &FieldsNamed,
) -> proc_macro2::TokenStream {
    let variant_ident = &variant.ident;

    let idents: Vec<_> = fields
        .named
        .iter()
        .map(|f| f.ident.clone().unwrap())
        .collect();
    let idents2 = idents.clone();

    // let types: Vec<_> = fields.named.iter().map(|f| f.ty.clone()).collect();
    let parsers: Vec<_> = fields.named.iter().map(gen_field_parser).collect();

    let expanded = quote! {
        do_parse!(
            #(
                #idents: #parsers >>
            )*
            (#name::#variant_ident { #(#idents2),* } )
        )
    };

    expanded
}

fn gen_enum_unnamed_fields(
    name: &Ident,
    variant: &Variant,
    fields: &FieldsUnnamed,
) -> proc_macro2::TokenStream {
    let variant_ident = &variant.ident;

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

    let parsers: Vec<_> = fields.unnamed.iter().map(gen_field_parser).collect();

    // println!("Parsers: {:#?}", parsers);

    let expanded = quote! {
        do_parse!(
            #(
                #idents: #parsers >>
            )*
            (#name::#variant_ident ( #(#idents2),* ) )
        )
    };

    // println!("Expanded: {}", expanded);

    expanded
}

fn gen_unnamed_fields(name: &Ident, fields: &FieldsUnnamed) -> proc_macro2::TokenStream {
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

    let parsers: Vec<_> = fields.unnamed.iter().map(gen_field_parser).collect();

    // println!("Parsers: {:#?}", parsers);

    let expanded = quote! {
        nom::do_parse!(input,
            #(
                #idents: #parsers >>
            )*
            (#name ( #(#idents2),* ) )
        )
    };

    // println!("Expanded: {}", expanded);

    expanded
}

fn gen_field_parser(field: &Field) -> proc_macro2::TokenStream {
    if field.attrs.is_empty() {
        let ty = &field.ty;

        let expanded = quote! {
            call!(<#ty>::parse)
        };

        expanded
    } else {
        let meta = &field.attrs[0].parse_meta().unwrap();

        let parser = match meta {
            Meta::List(list) => gen_parser_meta_list(list, field),
            m => unimplemented!("Meta"),
        };

        parser
    }
}

fn gen_parser_meta_list(list: &MetaList, field: &Field) -> proc_macro2::TokenStream {
    let kind: String = list.ident.to_string();

    match kind.as_str() {
        "tag" => {
            let (ints, len) = get_int_literals(&list);
            let ty = &field.ty;

            let tag = if len == 1 {
                quote! {
                    tag!(&[#ints])
                }
            } else {
                quote! {
                    tag![&[#(#ints),*]]
                }
            };

            let expanded = quote! {
                do_parse!(
                    #tag >>
                    value: call!(<#ty>::parse) >>
                    (value)
                )
            };

            expanded
        }
        "parser" => {
            let func = get_func_ident(&list);

            let expanded = quote! {
                call!(#func)
            };

            expanded
        }
        _ => unimplemented!("Unimplemented field attribute"),
    }
}

fn get_int_literals(list: &MetaList) -> (proc_macro2::TokenStream, usize) {
    let mut ints: Vec<syn::LitInt> = Vec::new();
    let mut iter = list.nested.iter();

    while let Some(NestedMeta::Literal(Lit::Int(int))) = iter.next() {
        ints.push(int.clone());
    }

    let len = ints.len();

    let expanded = if ints.len() == 1 {
        let val = &ints[0];
        quote! {
            #val
        }
    } else {
        quote! {
            &[#(#ints),*]
        }
    };

    (expanded, len)
}

fn get_func_ident(list: &MetaList) -> Ident {
    if let NestedMeta::Meta(Meta::Word(ident)) = list.nested.iter().next().unwrap() {
        ident.clone()
    } else {
        panic!("Invalid attribute func name")
    }
}

// fn gen_enum_impl(
//     name: &Ident,
//     attrs: &[Attribute],
//     generics: Generics,
//     data: DataEnum,
// ) -> proc_macro2::TokenStream {
//     // TODO: Handle Prefix Tagging

//     let variant_parsers: Vec<_> = data
//         .variants
//         .iter()
//         .map(|v| gen_variant_block(name, v))
//         .collect();

//     let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();;

//     let expanded = quote! {
//         impl #impl_generics crate::parser::Parse for #name #ty_generics #where_clause {
//             fn parse(input: &[u8]) -> crate::parser::PResult<Self> {
//                 // use log::debug;

//                 // debug!("Parsing {}", stringify!(#name));

//                 let (input, value) = nom::do_parse!(input,
//                     val: switch!(le_u8,
//                         #(#variant_parsers)|*
//                     ) >>
//                     (val)
//                 )?;

//                 // debug!("Parsed {}: {:?}", stringify!(#name), value);

//                 Ok((input, value))
//             }
//         }
//     };

//     // println!("{}", expanded);

//     expanded
// }

// fn gen_variant_block(enum_name: &Ident, variant: &Variant) -> proc_macro2::TokenStream {
//     let attr = &variant.attrs[0];

//     let variant_name = &variant.ident;

//     let path = &attr.path;
//     assert!(path.leading_colon.is_none());

//     let kind = path.segments.iter().next().unwrap();

//     let expanded = match kind.ident.to_string().as_str() {
//         "byte" => gen_byte_block(enum_name, variant_name, &variant.fields, attr.tts.clone()),
//         other => {
//             println!("{}", other);
//             panic!("Attr Path Block")
//         }
//     };

//     expanded
// }

// fn gen_byte_block(
//     enum_name: &Ident,
//     variant_name: &Ident,
//     fields: &Fields,
//     stream: proc_macro2::TokenStream,
// ) -> proc_macro2::TokenStream {
//     let lit: u8 = match stream.into_iter().next().unwrap() {
//         TokenTree::Group(g) => parse2::<LitInt>(g.stream())
//             .unwrap()
//             .value()
//             .try_into()
//             .unwrap(),
//         _ => panic!("Token Tree Panic"),
//     };

//     let resultant = match fields {
//         Fields::Unit => {
//             quote! {
//                 value!(#enum_name::#variant_name)
//             }
//         }
//         Fields::Unnamed(fields) => {
//             let field_tokens = gen_unnamed_fields(&fields);

//             quote!(call!(
//                 |mut input: &[u8]| {
//                     // let mut input = input;

//                     Ok((input, #enum_name::#variant_name(#field_tokens)))
//                 }
//             ))
//         }
//         Fields::Named(fields) => {
//             let field_tokens = gen_named_fields(&fields);

//             quote!(call!(
//                 |mut input: &[u8]| {
//                     // let mut input = input;

//                     Ok((input, #enum_name::#variant_name { #field_tokens }))
//                 }
//             ))
//         }

//         _ => panic!("Field Panic Enum"),
//     };

//     let match_arm = quote! {
//         #lit => #resultant
//     };

//     match_arm
// }
