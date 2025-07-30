#![feature(proc_macro_diagnostic)]

use syn::{parse_macro_input, DeriveInput, Data, Fields};
use quote::quote;


#[proc_macro_derive(Routable)]
pub fn route_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    if let Data::Struct(data) = input.data {
        if let Fields::Named(fields) = data.fields {
            let fields = fields.named.iter().map(|field| match &field.ident {
                Some(ident) => {
                    let key = ident.to_string();
                    let ty = &field.ty;

                    quote! {
                        if let Some(value) = map.get(#key).and_then(|value| (value.as_ref() as &dyn std::any::Any).downcast_ref::<String>().cloned()) {
                            new.insert(String::from(#key), std::rc::Rc::new(value) as std::rc::Rc<dyn ::puri::puri_core::component::tree::AttrValue>);
                        }
                    }
                },
                None => quote! {},
            });

            let name = input.ident;

            return proc_macro::TokenStream::from(quote! {
                impl ::puri_router::Routable for #name {
                    fn route(
                        map: std::collections::HashMap<String, std::rc::Rc<dyn ::puri::puri_core::component::tree::AttrValue>>
                    ) -> Option<std::collections::HashMap<String, std::rc::Rc<dyn ::puri::puri_core::component::tree::AttrValue>>> {
                        let mut new = HashMap::new();
                        #(#fields);*
                        Some(new)
                    }
                }
            });
        }
    }

    proc_macro::TokenStream::from(syn::Error::new(input.ident.span(), "You can only derive Routable for Structs with Named fields").to_compile_error())
}


