use crate::enums::html_element::HtmlElement;

use super::{AttributeValue, Html, Lex};
use proc_macro2::{Literal, Span, TokenStream};
use quote::quote;
use syn::{parse_str, Expr, Ident};

pub trait ToTokens {
    fn to_token_stream(&self) -> TokenStream;
}

impl ToTokens for Html {
    fn to_token_stream(&self) -> TokenStream {
        let mut stmts = Vec::<TokenStream>::new();
        self.collect_stmts(&mut stmts);

        quote! {
            {
                use std::fmt::Write;

                let html_escape = |input: &str| -> String {
                    let mut output = String::with_capacity(input.len());

                    for c in input.chars() {
                        match c {
                            '&' => output.push_str("&amp;"),
                            '"' => output.push_str("&quot;"),
                            '\'' => output.push_str("&#39;"),
                            '>' => output.push_str("&gt;"),
                            '<' => output.push_str("&lt;"),
                            _ => output.push(c),
                        }
                    }

                    output
                };

                let mut html = String::new();
                #(#stmts)*
                html
            }
        }
    }
}

impl Html {
    fn collect_stmts(&self, out: &mut Vec<TokenStream>) {
        let mut children = Vec::<TokenStream>::new();
        for child in &self.children {
            child.collect_stmts(&mut children);
        }

        match &self.value {
            Lex::Tag {
                element,
                attributes,
            } => {
                if *element != HtmlElement::Fragment {
                    let element_literal = element.to_string();

                    out.push(quote! {
                        html.push_str("<");
                        html.push_str(#element_literal);
                    });
                    for attr in attributes {
                        let key = attr.key();

                        match attr.value() {
                            Some(AttributeValue::Text(text)) => {
                                let text = Literal::string(&text);
                                out.push(quote! {
                                    write!(html, r#" {}="{}""#, #key, #text).unwrap();
                                });
                            }
                            Some(AttributeValue::Var(var)) => {
                                out.push(quote! {
                                    write!(html, r#" {}="{}""#, #key, #var).unwrap();
                                });
                            }
                            None => out.push(quote! {
                                write!(html, " {}", #key).unwrap();
                            }),
                        };
                    }

                    if element.is_void() {
                        out.push(quote! {
                            html.push_str(" />");
                        });
                    } else {
                        out.push(quote! {
                            html.push_str(">");
                        });
                    }
                }

                if !element.is_void() {
                    for child in &self.children {
                        child.collect_stmts(out);
                    }

                    if *element != HtmlElement::Fragment {
                        let tag = format!("</{}>\n", &element);
                        out.push(quote! {
                            html.push_str(#tag);
                        });
                    }
                }
            }
            Lex::SanitizedVar(var) => {
                let var: Expr =
                    parse_str(var).expect(&format!("Cannot parse the variable: {}", &var));

                out.push(quote! {
                    html.push_str(&html_escape(#var));
                });
            }
            Lex::Var(var) => {
                let var: Expr =
                    parse_str(var).expect(&format!("Cannot parse the variable: {}", &var));

                out.push(quote! {
                    html.push_str(#var);
                });
            }
            Lex::If(condition) => {
                let condition: Expr = parse_str(&condition)
                    .expect(&format!("Cannot parse the condition: {}", &condition));

                out.push(quote! {
                    if #condition {
                        #(#children)*
                    }
                });
            }
            Lex::ElseIf(condition) => {
                let condition: Expr = parse_str(&condition)
                    .expect(&format!("Cannot parse the condition: {}", &condition));

                out.push(quote! {
                    else if #condition {
                        #(#children)*
                    }
                });
            }
            Lex::Else => {
                out.push(quote! {
                    else {
                        #(#children)*
                    }
                });
            }
            Lex::For { var, iter } => {
                let var = Ident::new(var, Span::call_site());
                let iter: Expr =
                    parse_str(&iter).expect(&format!("Cannot parse the iter: {}", &iter));

                out.push(quote! {
                    for #var in #iter {
                        #(#children)*
                    }
                });
            }
            Lex::DocType => {
                out.push(quote! {
                    html.push_str("<!DOCTYPE html>\n");
                });
            }
        }
    }
}
