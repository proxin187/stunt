#![feature(proc_macro_diagnostic)]

mod codegen;
mod tags;
mod ir;

use syn::{parse_macro_input, DeriveInput, Data, Fields};
use quote::quote;

use codegen::HtmlBuilder;
use tags::Intermediate;
use ir::Ir;


#[proc_macro]
pub fn html(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut builder = HtmlBuilder::new();

    let intermediate: Intermediate = parse_macro_input!(input as Intermediate);

    let ir = Ir::new(&mut intermediate.tags.iter().peekable());

    proc_macro::TokenStream::from(builder.build(ir))
}

// TODO: this function was hacked together in a rush, refactor it so that its more readable
#[proc_macro_derive(Properties)]
pub fn properties_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    if let Data::Struct(data) = input.data {
        if let Fields::Named(fields) = data.fields {
            let mut builder_struct_fields: proc_macro2::TokenStream = proc_macro2::TokenStream::new();
            let mut builder_functions: proc_macro2::TokenStream = proc_macro2::TokenStream::new();
            let mut builder_token_type: proc_macro2::TokenStream = quote! { () };
            let mut buildable_fields: proc_macro2::TokenStream = proc_macro2::TokenStream::new();
            let mut build_fields: proc_macro2::TokenStream = proc_macro2::TokenStream::new();
            let mut field_markers: proc_macro2::TokenStream = proc_macro2::TokenStream::new();

            for field in fields.named.iter() {
                if let Some(ident) = &field.ident {
                    let name = &ident;
                    let ty = &field.ty;

                    builder_struct_fields.extend(quote! {
                        #name: Option<#ty>,
                    });

                    buildable_fields.extend(quote! {
                        #name: None,
                    });

                    let marker_name = syn::Ident::new(&format!("HasProp_{}", name), name.span());

                    field_markers.extend(quote! {
                        #[allow(non_camel_case_types)]
                        pub struct #marker_name<Token>(Token);
                    });

                    builder_functions.extend(quote! {
                        pub fn #name<Token>(&mut self, token: Token, value: #ty) -> #marker_name<Token> {
                            self.#name.replace(value);

                            #marker_name(token)
                        }
                    });

                    builder_token_type = quote! {
                        #marker_name<#builder_token_type>
                    };

                    build_fields.extend(quote! {
                        #name: self.#name.unwrap(),
                    });
                }
            }

            let name = input.ident;
            let builder_name = syn::Ident::new(&format!("_{}Builder", name), name.span());

            return proc_macro::TokenStream::from(quote! {
                impl stunt::component::Properties for #name {}

                impl stunt::component::Buildable for #name {
                    type Builder = #builder_name;

                    fn builder() -> Self::Builder {
                        #builder_name {
                            #buildable_fields
                        }
                    }
                }

                #field_markers

                pub struct #builder_name {
                    #builder_struct_fields
                }

                impl #builder_name {
                    #builder_functions

                    pub fn build(self, _token: #builder_token_type) -> #name {
                        #name {
                            #build_fields
                        }
                    }
                }
            });
        }
    }

    proc_macro::TokenStream::from(syn::Error::new(input.ident.span(), "You can only derive Properties for Structs with Named fields").to_compile_error())
}


