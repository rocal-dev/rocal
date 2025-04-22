use crate::{enums::html_element::HtmlElement, models::html_node::Node};
use proc_macro2::{TokenStream, TokenTree};
use std::{cell::RefCell, rc::Rc};
use syn::{
    braced,
    buffer::Cursor,
    parse::{Parse, ParseStream},
    token::Brace,
    Ident, LitStr, Token,
};

pub struct Lexer {
    root: Rc<RefCell<Node>>,
}

impl Lexer {
    pub fn get_root(&self) -> &Rc<RefCell<Node>> {
        &self.root
    }
}

impl Parse for Lexer {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let mut lexeme: Lexeme = input.parse()?;

        let root = match &lexeme {
            Lexeme::Tag {
                element,
                attributes,
                children: Some(children),
            } => {
                let node = Rc::new(RefCell::new(Node::Element {
                    element: element.clone(),
                    attributes: attributes
                        .iter()
                        .map(|attr| (attr.key.to_string(), attr.value.to_string()))
                        .collect(),
                    children: vec![],
                }));

                lexeme = syn::parse2(children.clone())?;

                dst(lexeme, node.clone())?;

                node
            }
            Lexeme::Tag {
                element,
                attributes,
                children: None,
            } => Rc::new(RefCell::new(Node::Element {
                element: element.clone(),
                attributes: attributes
                    .iter()
                    .map(|attr| (attr.key.to_string(), attr.value.to_string()))
                    .collect(),
                children: vec![],
            })),
            Lexeme::Text(text) => Rc::new(RefCell::new(Node::Text(text.to_string()))),
        };

        Ok(Lexer { root })
    }
}

fn dst(lex: Lexeme, parent: Rc<RefCell<Node>>) -> Result<Rc<RefCell<Node>>, syn::Error> {
    match lex {
        Lexeme::Tag {
            element,
            attributes,
            children: Some(children),
        } => {
            let node = Rc::new(RefCell::new(Node::Element {
                element: element.clone(),
                attributes: attributes
                    .iter()
                    .map(|attr| (attr.key.to_string(), attr.value.to_string()))
                    .collect(),
                children: vec![],
            }));

            let lexeme: Lexeme = syn::parse2(children.clone())?;

            let child_rc = dst(lexeme, node.clone())?;
            parent.borrow_mut().add_child(&child_rc.clone().borrow());

            Ok(parent.clone())
        }
        Lexeme::Tag {
            element,
            attributes,
            children: None,
        } => {
            let node = Rc::new(RefCell::new(Node::Element {
                element: element.clone(),
                attributes: attributes
                    .iter()
                    .map(|attr| (attr.key.to_string(), attr.value.to_string()))
                    .collect(),
                children: vec![],
            }));
            parent.borrow_mut().add_child(&node.clone().borrow());
            Ok(parent)
        }
        Lexeme::Text(text) => {
            let node = Rc::new(RefCell::new(Node::Text(text)));
            parent.borrow_mut().add_child(&node.clone().borrow());
            Ok(parent)
        }
    }
}

#[derive(Debug)]
enum Lexeme {
    Tag {
        element: HtmlElement,
        attributes: Vec<Attribute>,
        children: Option<TokenStream>,
    },
    Text(String),
}

impl Parse for Lexeme {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let mut tag: Option<Lexeme> = None;

        if input.peek(Token![<]) {
            let _: Token![<] = input.parse()?;

            if input.peek(Token![>]) {
                let _: Token![>] = input.parse()?;
                tag = Some(Lexeme::Tag {
                    element: HtmlElement::Fragment,
                    attributes: vec![],
                    children: None,
                });
            } else if input.peek(Ident) {
                let element: HtmlElement = input.parse()?;
                let mut attrs: Vec<Attribute> = vec![];

                while !input.peek(Token![>]) {
                    if input.peek(Ident) {
                        let attr: Attribute = input.parse()?;
                        attrs.push(attr);
                    }
                }

                tag = Some(Lexeme::Tag {
                    element: element.clone(),
                    attributes: attrs,
                    children: None,
                });

                let _: Token![>] = input.parse()?;

                let children = input.step(|cursor| {
                    return Lexeme::get_children(*cursor, Box::new(element));
                })?;

                tag = if let Some(Lexeme::Tag {
                    element,
                    attributes,
                    children: _,
                }) = tag
                {
                    Some(Lexeme::Tag {
                        element,
                        attributes,
                        children: Some(children),
                    })
                } else {
                    tag
                };
            } else {
                return Err(syn::Error::new(input.span(), "Invalid tag."));
            }
        } else if input.peek(Brace) {
            let content;
            braced!(content in input);
            let content: LitStr = content.parse()?;
            tag = Some(Lexeme::Text(content.value()));
        } else {
            return Err(syn::Error::new(input.span(), "Invalid syntax."));
        }

        return Ok(tag.unwrap_or(Lexeme::Tag {
            element: HtmlElement::Fragment,
            attributes: vec![],
            children: None,
        }));
    }
}

impl Lexeme {
    fn get_children(
        cursor: Cursor,
        element: Box<HtmlElement>,
    ) -> Result<(TokenStream, Cursor), syn::Error> {
        let mut rest = cursor;
        let mut children: Vec<TokenTree> = vec![];

        while let Some((tt, next)) = rest.token_tree() {
            match &tt {
                TokenTree::Punct(p) if p.as_char() == '<' => {}
                _ => {
                    children.push(tt);
                    rest = next;
                    continue;
                }
            };

            if let Some((tt2, next2)) = next.token_tree() {
                match &tt2 {
                    TokenTree::Punct(p) if p.as_char() == '/' => {}
                    _ => {
                        children.push(tt);
                        children.push(tt2);
                        rest = next2;
                        continue;
                    }
                }

                if let Some((tt3, next3)) = next2.token_tree() {
                    match &tt3 {
                        TokenTree::Ident(i) if i.to_string() == element.to_string() => {}
                        _ => {
                            children.push(tt);
                            children.push(tt2);
                            children.push(tt3);

                            rest = next3;
                            continue;
                        }
                    }

                    if let Some((tt4, next4)) = next3.token_tree() {
                        match &tt4 {
                            TokenTree::Punct(p) if p.as_char() == '>' => {
                                return Ok((children.into_iter().collect(), next4))
                            }
                            _ => {
                                children.push(tt);
                                children.push(tt2);
                                children.push(tt3);
                                children.push(tt4);

                                rest = next4;
                                continue;
                            }
                        }
                    }
                }
            }
        }

        Ok((children.into_iter().collect(), rest))
    }
}

#[derive(Debug, Clone)]
struct Attribute {
    key: String,
    value: String,
}

impl Attribute {
    pub fn new(key: &str, value: &str) -> Self {
        Self {
            key: key.to_string(),
            value: value.to_string(),
        }
    }
}

impl Parse for Attribute {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        if input.peek(Ident) {
            let key: Ident = input.parse()?;
            let _: Token![=] = input.parse()?;
            let value: LitStr = input.parse()?;

            return Ok(Attribute::new(&key.to_string(), &value.value()));
        }

        return Err(syn::Error::new(
            input.span(),
            "Some attributes should be here.",
        ));
    }
}
