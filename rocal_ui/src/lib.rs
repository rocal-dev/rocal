#![doc = include_str!("../README.md")]

pub mod data_types;
pub mod enums;
pub mod html;
pub mod html2;
pub mod lexer;
pub mod models;

use html::{lex_html, parse_html, parse_html2};
use proc_macro2::TokenStream;

// pub fn build_ui(item: TokenStream) -> TokenStream {
//     match lex_html(item.into()) {
//         Ok(html) => html.get_root().borrow().to_token_stream(None),
//         Err(err) => err.into_compile_error().into(),
//     }
// }

pub fn build_ui(item: TokenStream) -> TokenStream {
    match parse_html2(item.into()) {
        Ok(html) => {
            eprintln!("html2: {:#?}", html);
            TokenStream::new()
        }
        Err(err) => err.into_compile_error().into(),
    }
}
