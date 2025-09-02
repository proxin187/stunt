#![feature(proc_macro_diagnostic)]

mod routable;

use routable::Routable;

use syn::parse_macro_input;


#[proc_macro_derive(Routable, attributes(at, not_found))]
pub fn routable(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(tokens as Routable);

    proc_macro::TokenStream::from(input.tokens())
}


