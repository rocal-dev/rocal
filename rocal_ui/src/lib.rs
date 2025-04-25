#![doc = include_str!("../README.md")]

pub mod data_types;
pub mod enums;
pub mod html;
pub mod models;

use proc_macro2::TokenStream;

pub fn build_ui(item: TokenStream) -> TokenStream {
    match html::parse(item.into()) {
        Ok(html) => {
            eprintln!("html: {:#?}", html);
            TokenStream::new()
        }
        Err(err) => err.into_compile_error().into(),
    }
}
