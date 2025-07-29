#![feature(proc_macro_diagnostic)]

use syn::{parse_macro_input, DeriveInput, Data, Fields};
use quote::quote;


#[proc_macro_derive(Properties)]
pub fn properties_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    if let Data::Struct(data) = input.data {
        if let Fields::Named(fields) = data.fields {
            let fields = fields.named.iter().map(|field| match &field.ident {
                Some(ident) => {
                    let name = &ident;
                    let key = ident.to_string();
                    let ty = &field.ty;

                    quote! {
                        #name: attributes.get::<#ty>(#key).unwrap_or_default()
                    }
                },
                None => quote! {},
            });

            let name = input.ident;

            return proc_macro::TokenStream::from(quote! {
                impl ::puri::puri_core::component::Properties for #name {
                    fn create(attributes: AttrMap) -> Self {
                        #name {
                            #(#fields),*
                        }
                    }
                }
            });
        }
    }

    proc_macro::TokenStream::from(syn::Error::new(input.ident.span(), "You can only derive Properties for Structs with Named fields").to_compile_error())
}


