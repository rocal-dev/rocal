use crate::enums::html_element::HtmlElement;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{parse_str, Expr, Ident};

#[derive(Debug, Clone)]
pub enum Node {
    Element {
        element: HtmlElement,
        attributes: Vec<(String, String)>,
        children: Vec<Node>,
    },
    Text(String),
    Var(String),
    Expr(String),
    If {
        branches: Vec<Branch>,
    },
    For {
        var: String,
        iter: String,
        body: Box<Node>,
    },
}

impl Node {
    pub fn add_child(&mut self, child: &Node) {
        if let Node::Element {
            element: _,
            attributes: _,
            children,
        } = self
        {
            children.push(child.clone());
        }
    }

    pub fn to_token_stream(&self, html: Option<TokenStream>) -> TokenStream {
        let mut html = if let Some(html) = html {
            html
        } else {
            quote! {
                let mut html = String::new();
            }
        };

        match self {
            Self::Element {
                element,
                attributes,
                children,
            } => {
                if *element != HtmlElement::Fragment {
                    let mut attrs = String::new();

                    for (k, v) in attributes {
                        attrs += &format!(r#" {}="{}""#, k, v);
                    }

                    let tag = format!("<{}{}>\n", &element, &attrs);

                    html = quote! {
                        #html
                        html += #tag;
                    };
                }

                if !element.is_void() {
                    let mut nested = quote!();

                    for child in children {
                        nested = child.to_token_stream(Some(nested));
                    }

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
            Self::Text(text) => {
                html = quote! {
                    #html
                    html += #text;
                };
            }
            Self::Var(var) => {
                let var = Ident::new(var, Span::call_site());
                html = quote! {
                    #html
                    html += #var;
                };
            }
            Self::Expr(expr) => {
                let expr: Expr =
                    parse_str(expr).expect(&format!("Cannot parse the expression: {}", &expr));
                html = quote! {
                    #html
                    html += #expr;
                };
            }
            Self::If { branches } => {
                for (i, branch) in branches.iter().enumerate() {
                    let _last_idx = branches.len() - 1;

                    let body = &branch.body.to_token_stream(Some(quote!()));

                    match (i, &branch.cond) {
                        (0, Some(cond)) => {
                            let cond: Expr = parse_str(&cond)
                                .expect(&format!("Cannot parse the condition: {}", &cond));

                            html = quote! {
                                #html

                                if #cond {
                                    #body
                                }
                            }
                        }
                        (_last_idx, None) => {
                            html = quote! {
                                #html

                                else {
                                    #body
                                }
                            };
                        }
                        (_, Some(cond)) => {
                            let cond: Expr = parse_str(&cond)
                                .expect(&format!("Cannot parse the condition: {}", &cond));
                            html = quote! {
                                #html

                                else if #cond {
                                    #body
                                }
                            };
                        }
                        _ => {
                            panic!("Invalid condition")
                        }
                    }
                }
            }
            Self::For { var, iter, body } => {
                let var = Ident::new(var, Span::call_site());
                let iter = Ident::new(iter, Span::call_site());
                let body = &body.to_token_stream(Some(quote!()));

                html = quote! {
                    for #iter in #var {
                        #body
                    }
                };
            }
        }

        html
    }
}

#[derive(Debug, Clone)]
pub struct Branch {
    cond: Option<String>,
    body: Box<Node>,
}

impl Branch {
    pub fn new(cond: Option<String>, body: Box<Node>) -> Self {
        Self { cond, body }
    }
}
