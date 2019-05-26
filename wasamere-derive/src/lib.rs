extern crate proc_macro;

use crate::proc_macro::TokenStream;

use quote::quote;
use syn::{
    parse_macro_input, Data, DataStruct, DeriveInput, Fields, FieldsNamed, FieldsUnnamed,
    Generics, Ident,
};

#[proc_macro_derive(Parse)]
pub fn parse_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;
    let generics = input.generics;

    if let Data::Struct(data) = input.data {
        let expanded = gen_parse_impl(data, generics, name);

        TokenStream::from(expanded)
    } else {
        panic!()
    }
}

fn gen_parse_impl(data: DataStruct, generics: Generics, name: &Ident) -> proc_macro2::TokenStream {
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();;

    let fields = data.fields;

    let field_tokens = match fields {
        Fields::Named(ref fields) => gen_named_fields(fields),
        Fields::Unnamed(ref fields) => gen_unnamed_fields(fields),
        Fields::Unit => quote! {},
    };

    let self_return = match fields {
        Fields::Named(_) => quote! {
            Self { #field_tokens }
        },
        Fields::Unnamed(_) => quote! {
            Self ( #field_tokens )
        },
        Fields::Unit => quote! {
            Self ()
        },
    };

    let expanded = quote! {
        impl #impl_generics crate::parser::Parse for #name #ty_generics #where_clause {
            fn parse(input: &[u8]) -> crate::parser::PResult<Self> {
                let mut input = input;

                Ok((input, #self_return))
            }
        }
    };

    expanded
}

fn gen_parse_block() -> proc_macro2::TokenStream {
    let expanded = quote! {
        {
            let (new_input, value) = Parse::parse(input)?;
            input = new_input;
            value
        }
    };

    expanded
}

fn gen_named_fields(fields: &FieldsNamed) -> proc_macro2::TokenStream {
    // let gen = Vec::new();

    let idents: Vec<_> = fields
        .named
        .iter()
        .map(|f| f.ident.clone().unwrap())
        .collect();

    let blocks: Vec<_> = fields.named.iter().map(|_| gen_parse_block()).collect();

    let expanded = quote! {
        #(#idents: #blocks),*
    };

    expanded
}

fn gen_unnamed_fields(fields: &FieldsUnnamed) -> proc_macro2::TokenStream {
    let blocks: Vec<_> = fields.unnamed.iter().map(|_| gen_parse_block()).collect();

    let expanded = quote! {
        #(#blocks),*
    };

    expanded
}

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
// }
