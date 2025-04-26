use proc_macro2::{Delimiter, TokenStream, TokenTree};
use syn::{
    braced,
    buffer::Cursor,
    parse::{Parse, ParseBuffer, ParseStream},
    token::Brace,
    Expr, Ident, LitStr, Result, Token,
};

use crate::{data_types::stack::Stack, enums::html_element::HtmlElement};

pub mod to_tokens;

pub fn parse(item: TokenStream) -> Result<Html5> {
    Ok(syn::parse2(item.into())?)
}

#[derive(Clone, Debug)]
pub struct Html5 {
    children: Vec<Html5>,
    value: Lex,
}

impl Html5 {
    pub fn children(&self) -> &Vec<Html5> {
        &self.children
    }

    pub fn value(&self) -> &Lex {
        &self.value
    }
}

impl Parse for Html5 {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut stack: Stack<Html5> = Stack::new();

        stack.push(Html5 {
            children: vec![],
            value: Lex::Tag {
                element: HtmlElement::Fragment,
                attributes: vec![],
            },
        });

        while !input.is_empty() {
            if input.peek(Token![<]) && input.peek2(Ident) {
                input.parse::<Token![<]>()?;
                let element: HtmlElement = input.parse()?;
                let mut attrs: Vec<Attribute> = vec![];

                while !(input.peek(Token![>]) || input.peek(Token![/])) {
                    if input.peek(Ident) || input.peek(Token![type]) {
                        let attr: Attribute = input.parse()?;
                        attrs.push(attr);
                    }
                }

                if input.peek(Token![/]) {
                    input.parse::<Token![/]>()?;
                }

                input.parse::<Token![>]>()?;

                let tag = Html5 {
                    children: vec![],
                    value: Lex::Tag {
                        element: element.clone(),
                        attributes: attrs,
                    },
                };

                if element.is_void() {
                    if let Some(mut parent) = stack.pop() {
                        parent.children.push(tag);
                        stack.push(parent);
                    } else {
                        stack.push(tag);
                    }
                } else {
                    stack.push(tag);
                }
            } else if input.peek(Token![<]) && input.peek2(Token![/]) && input.peek3(Ident) {
                input.parse::<Token![<]>()?;
                input.parse::<Token![/]>()?;

                let el: HtmlElement = input.parse()?;

                let previous1 = if let Some(previous1) = stack.pop() {
                    previous1
                } else {
                    return Err(syn::Error::new(
                        input.span(),
                        &format!("There is no opening tag for </{}>", &el.to_string()),
                    ));
                };

                if let Lex::Tag { element, .. } = &previous1.value {
                    if element.to_string() == el.to_string() {
                        if let Some(mut previous2) = stack.pop() {
                            previous2.children.push(previous1);
                            stack.push(previous2);
                        } else {
                            return Err(syn::Error::new(
                                input.span(),
                                "A single root is mandatory.",
                            ));
                        }
                    } else {
                        return Err(syn::Error::new(input.span(), "Invalid syntax"));
                    }
                }

                input.parse::<Token![>]>()?;
            } else if input.peek(Brace) {
                let content;
                braced!(content in input);

                if !content.peek(Brace) {
                    let text: LitStr = content.parse()?;

                    let text = Html5 {
                        children: vec![],
                        value: Lex::Text(text.value()),
                    };

                    if let Some(mut parent) = stack.pop() {
                        parent.children.push(text);
                        stack.push(parent);
                    } else {
                        return Err(syn::Error::new(input.span(), "A single root is mandatory"));
                    }
                } else {
                    let var;
                    braced!(var in content);

                    let var = Self::extract_variable(&var)?;

                    if let Some(mut parent) = stack.pop() {
                        parent.children.push(Html5 {
                            children: vec![],
                            value: Lex::Var(var),
                        });
                        stack.push(parent);
                    } else {
                        return Err(syn::Error::new(input.span(), "A single root is mandatory"));
                    }
                }
            } else if input.peek(Token![if]) {
                input.parse::<Token![if]>()?;

                let condition = Self::extract_condition(input)?;

                let body;
                braced!(body in input);
                let body: ParseStream = &body;

                let body: Html5 = Self::parse(&body)?;

                if let Some(mut previous) = stack.pop() {
                    previous.children.push(Html5 {
                        children: vec![body],
                        value: Lex::If(condition.to_string()),
                    });
                    stack.push(previous);
                } else {
                    return Err(syn::Error::new(
                        input.span(),
                        "`if` should be used inside of a node",
                    ));
                }
            } else if input.peek(Token![else]) && input.peek2(Token![if]) {
                input.parse::<Token![else]>()?;
                input.parse::<Token![if]>()?;

                let condition = Self::extract_condition(input)?;

                let body;
                braced!(body in input);
                let body: ParseStream = &body;

                let body: Html5 = Self::parse(&body)?;

                if let Some(mut previous) = stack.pop() {
                    previous.children.push(Html5 {
                        children: vec![body],
                        value: Lex::ElseIf(condition.to_string()),
                    });
                    stack.push(previous);
                } else {
                    return Err(syn::Error::new(
                        input.span(),
                        "`else-if` should be used inside of a node",
                    ));
                }
            } else if input.peek(Token![else]) {
                input.parse::<Token![else]>()?;

                let body;
                braced!(body in input);
                let body: ParseStream = &body;

                let body: Html5 = Self::parse(&body)?;

                if let Some(mut previous) = stack.pop() {
                    previous.children.push(Html5 {
                        children: vec![body],
                        value: Lex::Else,
                    });
                    stack.push(previous);
                } else {
                    return Err(syn::Error::new(
                        input.span(),
                        "`else` should be used inside of a node",
                    ));
                }
            } else if input.peek(Token![for]) {
                input.parse::<Token![for]>()?;

                let var: Ident = input.parse()?;

                input.parse::<Token![in]>()?;

                let iter = Self::extract_iter(input)?;

                let body;
                braced!(body in input);
                let body: ParseStream = &body;

                let body: Html5 = Self::parse(&body)?;

                if let Some(mut previous) = stack.pop() {
                    previous.children.push(Html5 {
                        children: vec![body],
                        value: Lex::For {
                            var: var.to_string(),
                            iter: iter.to_string(),
                        },
                    });
                    stack.push(previous);
                } else {
                    return Err(syn::Error::new(
                        input.span(),
                        "`for-in` should be used inside of a node",
                    ));
                }
            } else {
                return Err(syn::Error::new(input.span(), "Invalid token"));
            }
        }

        if stack.len != 1 {
            return Err(syn::Error::new(
                input.span(),
                "Error: lack of some closing tags",
            ));
        }

        let root = if let Some(root) = stack.pop() {
            root
        } else {
            return Err(syn::Error::new(input.span(), "There is no root tag"));
        };

        Ok(root)
    }
}

impl Html5 {
    fn extract_variable(input: &ParseBuffer) -> Result<String> {
        let variable = input.step(|cursor| {
            let result: Result<(String, Cursor)> = {
                let mut rest = *cursor;
                let mut tokens = String::new();

                while let Some((tt, next)) = rest.token_tree() {
                    tokens += &tt.to_string();
                    rest = next;
                }

                Ok((tokens, rest))
            };

            result
        })?;

        Ok(variable)
    }

    fn extract_iter(input: ParseStream) -> Result<TokenStream> {
        let iter = input.step(|cursor| {
            let result: Result<(TokenStream, Cursor)> = {
                let mut rest = *cursor;
                let mut tokens: Vec<TokenTree> = vec![];

                while let Some((tt, next)) = rest.token_tree() {
                    if let TokenTree::Group(g) = &tt {
                        if g.delimiter() == Delimiter::Brace {
                            return Ok((tokens.into_iter().collect(), rest));
                        }
                    }

                    tokens.push(tt);
                    rest = next;
                }

                if tokens.is_empty() {
                    Err(syn::Error::new(input.span(), "Iter should be here."))
                } else {
                    Ok((tokens.into_iter().collect(), *cursor))
                }
            };

            result
        });

        iter
    }

    fn extract_condition(input: ParseStream) -> Result<TokenStream> {
        let condition = input.step(|cursor| {
            let result: Result<(TokenStream, Cursor)> = {
                let mut rest = *cursor;
                let mut tokens: Vec<TokenTree> = vec![];

                while let Some((tt, next)) = rest.token_tree() {
                    if let TokenTree::Group(g) = &tt {
                        if g.delimiter() == Delimiter::Brace {
                            return Ok((tokens.into_iter().collect(), rest));
                        }
                    }

                    tokens.push(tt);
                    rest = next;
                }

                if tokens.is_empty() {
                    Err(syn::Error::new(input.span(), "Condition shuold be here."))
                } else {
                    Ok((tokens.into_iter().collect(), *cursor))
                }
            };

            result
        });

        condition
    }
}

#[derive(Clone, Debug)]
pub enum Lex {
    Tag {
        element: HtmlElement,
        attributes: Vec<Attribute>,
    },
    Text(String),
    Var(String),
    If(String),
    ElseIf(String),
    Else,
    For {
        var: String,
        iter: String,
    },
}

#[derive(Debug, Clone)]
pub struct Attribute(String, AttributeValue);

#[derive(Debug, Clone)]
pub enum AttributeValue {
    Text(String),
    Var(Expr),
}

impl Parse for Attribute {
    fn parse(input: ParseStream) -> Result<Self> {
        let key = if input.peek(Ident) {
            let key: Ident = input.parse()?;
            key.to_string()
        } else if input.peek(Token![type]) {
            input.parse::<Token![type]>()?;
            "type".to_string()
        } else {
            return Err(syn::Error::new(
                input.span(),
                "Some attributes should be here.",
            ));
        };

        input.parse::<Token![=]>()?;

        if input.peek(Brace) {
            let mut value;
            braced!(value in input);
            braced!(value in value);
            let value: Expr = value.parse()?;
            return Ok(Attribute(key, AttributeValue::Var(value)));
        }

        let value: LitStr = input.parse()?;
        return Ok(Attribute(key, AttributeValue::Text(value.value())));
    }
}

impl Attribute {
    pub fn key(&self) -> &str {
        &self.0
    }
    pub fn value(&self) -> &AttributeValue {
        &self.1
    }
}
