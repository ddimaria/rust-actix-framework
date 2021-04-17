extern crate proc_macro;
extern crate proc_macro2;

mod timestamps;

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn timestamps(_attr: TokenStream, item: TokenStream) -> TokenStream {
    timestamps::add(_attr, item)
}
