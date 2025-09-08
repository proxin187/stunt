#![feature(proc_macro_diagnostic)]

mod properties;
mod service;
mod html;

use syn::{parse_macro_input, DeriveInput, Data, Fields};
use quote::quote;

use html::codegen::HtmlBuilder;
use html::tags::Intermediate;
use html::intermediate::Ir;

use properties::{Field, BuilderFields, BuilderFunctions, BuilderTokenType, BuilderFieldsInit, BuilderMarkers, BuilderFieldsBuild, BuilderChildren};


#[proc_macro]
pub fn html(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut builder = HtmlBuilder::new();

    let intermediate: Intermediate = parse_macro_input!(input as Intermediate);
    let mut tags = intermediate.tags.iter().peekable();

    let ir = Ir::new(&mut tags);

    match tags.next() {
        Some(tag) => proc_macro::TokenStream::from(syn::Error::new(tag.span().into(), "Only one root node is allowed").to_compile_error()),
        None => proc_macro::TokenStream::from(builder.build(ir)),
    }
}

#[proc_macro_derive(Properties)]
pub fn properties(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    if let Data::Struct(data) = input.data {
        if let Fields::Named(fields) = data.fields {
            let fields = fields.named.into_iter()
                .filter_map(|field| field.ident.map(|ident| Field::new(ident, field.ty)))
                .collect::<Vec<Field>>();

            let builder_fields = BuilderFields::new(&fields);
            let builder_functions = BuilderFunctions::new(&fields);
            let builder_token_type = BuilderTokenType::new(&fields);
            let builder_fields_init = BuilderFieldsInit::new(&fields);
            let builder_markers = BuilderMarkers::new(&fields);
            let builder_fields_build = BuilderFieldsBuild::new(&fields);
            let builder_children = BuilderChildren::new(&fields);

            let name = input.ident;
            let builder_name = syn::Ident::new(&format!("_{}Builder", name), name.span());

            return proc_macro::TokenStream::from(quote! {
                impl ::stunt::frontend::Buildable for #name {
                    type Builder = #builder_name;

                    fn builder() -> Self::Builder {
                        #builder_name {
                            #builder_fields_init
                        }
                    }
                }

                #builder_markers

                #[allow(missing_docs)]
                pub struct #builder_name {
                    #builder_fields
                }

                impl ::stunt::frontend::PreBuild for #builder_name {
                    #builder_children

                    fn build(&self) -> ::std::rc::Rc<dyn ::std::any::Any> {
                        ::std::rc::Rc::new(#name {
                            #builder_fields_build
                        })
                    }
                }

                impl #builder_name {
                    #builder_functions

                    #[allow(missing_docs)]
                    pub fn typecheck(&self, _token: #builder_token_type) {}
                }
            });
        }
    }

    proc_macro::TokenStream::from(syn::Error::new(input.ident.span(), "You can only derive Properties for Structs with Named fields").to_compile_error())
}

#[proc_macro_attribute]
pub fn service(input: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    item
}

#[proc_macro_attribute]
pub fn stunt_main(input: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    item
}


