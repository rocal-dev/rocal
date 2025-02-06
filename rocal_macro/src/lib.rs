use proc_macro::TokenStream;
use rocal_core::{build_action, build_config, build_route, start_app};

#[proc_macro_attribute]
pub fn main(_: TokenStream, item: TokenStream) -> TokenStream {
    start_app(item.into()).into()
}

#[proc_macro_attribute]
pub fn action(_: TokenStream, item: TokenStream) -> TokenStream {
    build_action(item.into()).into()
}

#[proc_macro]
pub fn route(item: TokenStream) -> TokenStream {
    build_route(item.into()).into()
}

#[proc_macro]
pub fn config(item: TokenStream) -> TokenStream {
    build_config(item.into()).into()
}
