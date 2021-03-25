use proc_macro::{TokenStream};
use syn::{parse_macro_input, DeriveInput, Data, DataStruct, Fields, FieldsNamed};
use quote::quote;
use proc_macro2::{Ident, Span};

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let type_name = input.ident;
    let builder_name = Ident::new(&format!("{}Builder", type_name), Span::call_site());

    let fields_iter = match input.data {
        Data::Struct(
            DataStruct {
                fields: Fields::Named(
                    FieldsNamed { ref named, .. }
                ), ..
            }
        ) => named.iter(),
        _ => unimplemented!(),
    };

    let builder_fields = fields_iter
        .clone()
        .map(|field| {
            let ident = &field.ident;
            let ty = &field.ty;
            quote! { #ident: Option<#ty> }
        });
    let init_fields = fields_iter
        .clone()
        .map(|field| {
            let ident = &field.ident;
            quote! { #ident: None }
        });

    let expanded = quote! {
        pub struct #builder_name {
            #(#builder_fields),*
        }

        impl #type_name {
            pub fn builder() -> #builder_name {
                #builder_name {
                    #(#init_fields),*
                }
            }
        }
    };

    TokenStream::from(expanded)
}
