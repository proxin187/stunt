mod tags;

use proc_macro::TokenStream;
use syn::parse_macro_input;
use quote::quote;

use tags::{Intermediate, Tag};


fn generate(tags: Vec<Tag>) -> TokenStream {
    if let Some(tag) = tags.first() {
    }

    TokenStream::new()
}

#[proc_macro]
pub fn html(input: TokenStream) -> TokenStream {
    let intermediate: Intermediate = parse_macro_input!(input as Intermediate);

    // TODO: the next thing to do is to iterate over the tags and check for mismatches
    // finally we need to generate the output code

    generate(intermediate.tags)
}


