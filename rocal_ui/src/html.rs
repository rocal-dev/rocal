use std::{cell::RefCell, rc::Rc, str::FromStr};

use crate::{
    data_types::queue::Queue, enums::html_element::HtmlElement, html2::Html2, lexer::Lexer,
    models::html_node::Node,
};
use proc_macro2::{TokenStream, TokenTree};
use syn::{
    parse::{Parse, ParseStream},
    Ident, LitStr, Token,
};

pub fn parse_html(item: TokenStream) -> Result<Html, syn::Error> {
    let result: Html = syn::parse2(item.into())?;
    Ok(result)
}

pub fn lex_html(item: TokenStream) -> Result<Lexer, syn::Error> {
    let result: Lexer = syn::parse2(item.into())?;
    Ok(result)
}

pub fn parse_html2(item: TokenStream) -> Result<Html2, syn::Error> {
    let result: Html2 = syn::parse2(item.into())?;
    Ok(result)
}

pub struct Html {
    root: Node,
}

impl Html {
    pub fn new(root: Node) -> Self {
        Self { root }
    }

    pub fn get_root(&self) -> &Node {
        &self.root
    }
}

impl Parse for Html {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let tag: Tag = input.parse()?;
        let root = Node::Element {
            element: tag.element.clone(),
            attributes: tag
                .attributes
                .iter()
                .map(|attr| (attr.key.to_string(), attr.value.to_string()))
                .collect(),
            children: vec![],
        };
        let pointer = Rc::new(RefCell::new(&root));
        let mut queue: Queue<Tag> = Queue::new();
        queue.enqueue(tag);

        while let Some(tag) = queue.dequeue() {
            for child in &tag.children {
                let tag: Tag = syn::parse2(child.clone())?;
                queue.enqueue(tag);
            }
        }

        Ok(Html::new(Node::Text(String::new())))
    }
}

#[derive(Debug, Clone)]
struct Tag {
    element: HtmlElement,
    attributes: Vec<Attribute>,
    children: Vec<TokenStream>,
}

impl Tag {
    pub fn new(element: HtmlElement, attributes: Vec<Attribute>) -> Self {
        Self {
            element,
            attributes,
            children: vec![],
        }
    }
}

impl Tag {
    fn parse_opening_tag(input: &ParseStream) -> Result<Self, syn::Error> {
        if input.peek(Token![>]) {
            let _: Token![>] = input.parse()?;
            return Ok(Tag::new(HtmlElement::Fragment, vec![]));
        };

        if input.peek(Ident) {
            let element: HtmlElement = input.parse()?;
            let mut attrs: Vec<Attribute> = vec![];

            while !input.peek(Token![>]) {
                if input.peek(Ident) {
                    let attr: Attribute = input.parse()?;
                    attrs.push(attr);
                }
            }

            return Ok(Tag::new(element, attrs));
        }

        Ok(Tag::new(HtmlElement::Fragment, vec![]))
    }
}

impl Parse for Tag {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let mut children = String::new();

        if input.peek(Token![<]) {
            let _: Token![<] = input.parse()?;

            let mut tag = Self::parse_opening_tag(&input)?;

            let _: Token![>] = input.parse()?;

            let _ = input.step(|cur| {
                let mut rest = *cur;

                while let Some((tt, next)) = rest.token_tree() {
                    match &tt {
                        TokenTree::Punct(p) if p.as_char() == '<' => {
                            if let Some((tt2, next2)) = next.token_tree() {
                                match &tt2 {
                                    TokenTree::Punct(p) if p.as_char() == '/' => {
                                        if let Some((tt3, next3)) = next2.token_tree() {
                                            match &tt3 {
                                                TokenTree::Ident(i)
                                                    if i.to_string() == tag.element.to_string() =>
                                                {
                                                    if let Some((tt4, next4)) = next3.token_tree() {
                                                        match &tt4 {
                                                            TokenTree::Punct(p)
                                                                if p.as_char() == '>' =>
                                                            {
                                                                return Ok(((), next4));
                                                            }
                                                            _ => {
                                                                children += &tt.to_string();
                                                                children += &tt2.to_string();
                                                                children += &tt3.to_string();
                                                                children += &tt4.to_string();
                                                                rest = next4;
                                                            }
                                                        }
                                                    }
                                                }
                                                _ => {
                                                    children += &tt.to_string();
                                                    children += &tt2.to_string();
                                                    children += &tt3.to_string();
                                                    rest = next3;
                                                }
                                            }
                                        }
                                    }
                                    _ => {
                                        children += &tt.to_string();
                                        children += &tt2.to_string();
                                        rest = next2;
                                    }
                                }
                            }
                        }
                        _ => {
                            children += &tt.to_string();
                            rest = next;
                        }
                    }
                }

                Err(cur.error(&format!("expected `</{}>`", tag.element)))
            })?;

            tag.children.push(TokenStream::from_str(&children)?);

            Ok(tag)
        } else {
            return Err(syn::Error::new(
                input.span(),
                "The macro always requires a single root node",
            ));
        }
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
