#![doc = include_str!("../README.md")]

pub mod data_types;
pub mod enums;
pub mod html;
pub mod html5;
pub mod models;

use html::to_tokens::ToTokens;
use proc_macro2::TokenStream;

pub fn build_ui(item: TokenStream) -> TokenStream {
    match html::parse(item.into()) {
        Ok(html) => html.to_token_stream().into(),
        Err(err) => err.into_compile_error().into(),
    }
}
