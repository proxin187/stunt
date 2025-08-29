use quote::{quote, ToTokens};
use syn::{Ident, Type};


pub struct Field {
    ident: Ident,
    marker_ident: Ident,
    ty: Type,
}

impl Field {
    pub fn new(ident: Ident, ty: Type) -> Field {
        Field {
            ident: ident.clone(),
            marker_ident: syn::Ident::new(&format!("HasProp_{}", ident), ident.span()),
            ty,
        }
    }
}

pub struct BuilderFields<'a> {
    fields: &'a [Field],
}

impl<'a> BuilderFields<'a> {
    pub fn new(fields: &'a [Field]) -> BuilderFields<'a> {
        BuilderFields {
            fields,
        }
    }
}

impl<'a> ToTokens for BuilderFields<'a> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        for field in self.fields.iter() {
            let Field { ident, ty, .. } = &field;

            tokens.extend(quote! {
                #ident: Option<#ty>,
            });
        }
    }
}

pub struct BuilderFunctions<'a> {
    fields: &'a [Field],
}

impl<'a> BuilderFunctions<'a> {
    pub fn new(fields: &'a [Field]) -> BuilderFunctions<'a> {
        BuilderFunctions {
            fields,
        }
    }
}

impl<'a> ToTokens for BuilderFunctions<'a> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        for field in self.fields.iter() {
            let Field { ident, marker_ident, ty } = &field;

            tokens.extend(quote! {
                #[allow(missing_docs)]
                pub fn #ident<Token>(&mut self, token: Token, value: #ty) -> #marker_ident<Token> {
                    self.#ident.replace(value);

                    #marker_ident(token)
                }
            });
        }
    }
}

pub struct BuilderTokenType<'a> {
    fields: &'a [Field],
}

impl<'a> BuilderTokenType<'a> {
    pub fn new(fields: &'a [Field]) -> BuilderTokenType<'a> {
        BuilderTokenType {
            fields,
        }
    }
}

impl<'a> ToTokens for BuilderTokenType<'a> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let mut token_type = quote! { () };

        for field in self.fields.iter() {
            let Field { marker_ident, .. } = &field;

            token_type = quote! { #marker_ident<#token_type> };
        }

        tokens.extend(quote! { #token_type });
    }
}

pub struct BuilderFieldsInit<'a> {
    fields: &'a [Field],
}

impl<'a> BuilderFieldsInit<'a> {
    pub fn new(fields: &'a [Field]) -> BuilderFieldsInit<'a> {
        BuilderFieldsInit {
            fields,
        }
    }
}

impl<'a> ToTokens for BuilderFieldsInit<'a> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        for field in self.fields.iter() {
            let Field { ident, .. } = &field;

            tokens.extend(quote! {
                #ident: None,
            });
        }
    }
}

pub struct BuilderMarkers<'a> {
    fields: &'a [Field],
}

impl<'a> BuilderMarkers<'a> {
    pub fn new(fields: &'a [Field]) -> BuilderMarkers<'a> {
        BuilderMarkers {
            fields,
        }
    }
}

impl<'a> ToTokens for BuilderMarkers<'a> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        for field in self.fields.iter() {
            let Field { marker_ident, .. } = &field;

            tokens.extend(quote! {
                #[allow(non_camel_case_types, missing_docs)]
                pub struct #marker_ident<Token>(Token);
            });
        }
    }
}

pub struct BuilderFieldsBuild<'a> {
    fields: &'a [Field],
}

impl<'a> BuilderFieldsBuild<'a> {
    pub fn new(fields: &'a [Field]) -> BuilderFieldsBuild<'a> {
        BuilderFieldsBuild {
            fields,
        }
    }
}

impl<'a> ToTokens for BuilderFieldsBuild<'a> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        for field in self.fields.iter() {
            let Field { ident, .. } = &field;

            tokens.extend(quote! {
                #ident: self.#ident.unwrap(),
            });
        }
    }
}


