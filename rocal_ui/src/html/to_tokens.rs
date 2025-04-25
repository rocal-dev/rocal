use crate::enums::html_element::HtmlElement;

use super::{Html, Lex};
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{parse_str, Expr, Ident};

pub trait ToTokens {
    fn to_token_stream(&self, html: Option<TokenStream>) -> TokenStream;
}

impl ToTokens for Html {
    fn to_token_stream(&self, html: Option<TokenStream>) -> TokenStream {
        let mut html = if let Some(html) = html {
            html
        } else {
            quote! {
                let mut html = String::new();
            }
        };

        let Html { children, value } = self;

        let mut nested = quote!();

        for child in children {
            nested = child.to_token_stream(Some(nested));
        }

        match &value {
            Lex::Tag {
                element,
                attributes,
            } => {
                if *element != HtmlElement::Fragment {
                    let mut attrs = String::new();

                    for attr in attributes {
                        attrs += &format!(r#" {}="{}""#, attr.key(), attr.value());
                    }

                    let tag = format!("<{}{}>\n", &element, &attrs);

                    html = quote! {
                        #html
                        html += #tag;
                    };
                }

                if !element.is_void() {
                    html = quote! {
                        #html
                        #nested
                    };

                    if *element != HtmlElement::Fragment {
                        let tag = format!("</{}>\n", &element);
                        html = quote! {
                            #html
                            html += #tag;
                        };
                    }
                }
            }
            Lex::Text(text) => {
                html = quote! {
                    #html
                    html += #text;
                };
            }
            Lex::Var(var) => {
                let var = Ident::new(var, Span::call_site());
                html = quote! {
                    #html
                    html += #var;
                };
            }
            Lex::If(condition) => {
                let condition: Expr = parse_str(&condition)
                    .expect(&format!("Cannot parse the condition: {}", &condition));

                html = quote! {
                    #html

                    if #condition {
                        #nested
                    }
                };
            }
            Lex::ElseIf(condition) => {
                let condition: Expr = parse_str(&condition)
                    .expect(&format!("Cannot parse the condition: {}", &condition));

                html = quote! {
                    #html
                    else if #condition {
                        #nested
                    }
                };
            }
            Lex::Else => {
                html = quote! {
                    #html
                    else {
                        #nested
                    }
                };
            }
            Lex::For { var, iter } => {
                let var = Ident::new(var, Span::call_site());
                let iter = Ident::new(iter, Span::call_site());

                html = quote! {
                    for #var in #iter {
                        #nested
                    }
                };
            }
        }

        html
    }
}
