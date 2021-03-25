use proc_macro::{TokenStream};
use syn::{parse_macro_input, DeriveInput, Data, DataStruct, Fields, FieldsNamed, Field};
use quote::{quote, format_ident};
use proc_macro2::{TokenStream as TokenStream2, Ident};

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let struct_name = input.ident;
    let builder_name = format_ident!("{}Builder", struct_name);

    let fields = extract_fields(&input.data);

    let builder_fields = expand_builder_fields(&fields);
    let init_builder_fields = expand_init_builder_fields(&fields);
    let setters = expand_setters(&fields);
    let build_method = expand_build_method(&fields, &struct_name);

    let expanded = quote! {
        pub struct #builder_name {
            #builder_fields
        }

        impl #builder_name {
            #setters
            #build_method
        }

        impl #struct_name {
            pub fn builder() -> #builder_name {
                #builder_name {
                    #init_builder_fields
                }
            }
        }
    };
    TokenStream::from(expanded)
}

fn extract_fields(data: &Data) -> &FieldsNamed {
    match *data {
        Data::Struct(
            DataStruct {
                fields: Fields::Named(
                    ref fields
                ), ..
            }
        ) => fields,
        _ => unimplemented!(),
    }
}

fn map_fields<F>(fields: &FieldsNamed, split: bool, func: F) -> TokenStream2
    where F: Fn(&Field) -> TokenStream2
{
    let iter = fields.named.iter().map(func);
    if split {
        quote! { #(#iter),* }
    } else {
        quote! { #(#iter)* }
    }
}

fn expand_builder_fields(fields: &FieldsNamed) -> TokenStream2 {
    map_fields(fields, true, |field| {
        let ident = &field.ident;
        let ty = &field.ty;
        quote! { #ident: Option<#ty> }
    })
}

fn expand_init_builder_fields(fields: &FieldsNamed) -> TokenStream2 {
    map_fields(fields, true, |field| {
        let ident = &field.ident;
        quote! { #ident: None }
    })
}

fn expand_build_method(fields: &FieldsNamed, struct_name: &Ident) -> TokenStream2 {
    let init_struct_fields = map_fields(fields, true, |field| {
        let ident = &field.ident;
        let err_msg = format!("field {} missing!", ident.as_ref().unwrap());
        quote! { #ident: self.#ident.clone().ok_or(#err_msg)? }
    });
    quote! {
        pub fn build(&mut self) -> Result<#struct_name, Box<dyn std::error::Error>> {
            Ok(#struct_name {
                #init_struct_fields
            })
        }
    }
}


fn expand_setters(fields: &FieldsNamed) -> TokenStream2 {
    map_fields(fields, false, |field| {
        let ident = &field.ident;
        let ty = &field.ty;
        quote! {
            pub fn #ident(&mut self, #ident: #ty) -> &mut Self {
                self.#ident = Some(#ident);
                self
            }
         }
    })
}
