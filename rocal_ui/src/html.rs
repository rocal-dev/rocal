use crate::{data_types::stack::Stack, enums::html_element::HtmlElement};
use proc_macro2::{Delimiter, TokenStream, TokenTree};
use syn::{
    braced,
    buffer::Cursor,
    parse::{Parse, ParseBuffer, ParseStream, Parser, Result},
    token::Brace,
    Expr, Ident, LitStr, Token,
};

pub mod to_tokens;

pub fn parse(item: TokenStream) -> Result<Html> {
    Ok(syn::parse2(item.into())?)
}

#[derive(Clone, Debug)]
pub struct Html {
    children: Vec<Html>,
    value: Lex,
}

impl Parse for Html {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut root = Html {
            children: vec![],
            value: Lex::Tag {
                element: HtmlElement::Fragment,
                attributes: vec![],
            },
        };
        Ok(Self::parse_html(&input, &mut root)?)
    }
}

impl Html {
    pub fn children(&self) -> &Vec<Html> {
        &self.children
    }

    pub fn value(&self) -> &Lex {
        &self.value
    }

    fn parse_html(input: &ParseStream, html: &mut Html) -> Result<Html> {
        if input.is_empty() {
            return Ok(html.clone());
        }

        if input.peek(Token![<]) {
            input.parse::<Token![<]>()?;

            if input.peek(Ident) {
                let element: HtmlElement = input.parse()?;
                let mut attrs: Vec<Attribute> = vec![];

                while !(input.peek(Token![>]) || input.peek(Token![/])) {
                    if input.peek(Ident) {
                        let attr: Attribute = input.parse()?;
                        attrs.push(attr);
                    }
                }

                if input.peek(Token![/]) {
                    input.parse::<Token![/]>()?;
                }

                input.parse::<Token![>]>()?;

                if element.is_void() {
                    html.children.push(Html {
                        children: vec![],
                        value: Lex::Tag {
                            element,
                            attributes: attrs,
                        },
                    });
                } else {
                    let next_input = Self::get_next_input(&input, &element)?;

                    let mut new_html = Html {
                        children: vec![],
                        value: Lex::Tag {
                            element,
                            attributes: attrs,
                        },
                    };

                    if let Some(next_input) = next_input {
                        let parser = |input: ParseStream| Self::parse_html(&input, &mut new_html);
                        parser.parse2(next_input)?;
                    }

                    html.children.push(new_html);
                }
            } else {
                return Err(syn::Error::new(input.span(), "The syntax is invalid"));
            }
        } else if input.peek(Brace) {
            let mut content;
            braced!(content in input);

            if let Ok(content) = content.parse::<LitStr>() {
                html.children.push(Html {
                    children: vec![],
                    value: Lex::Text(content.value()),
                });
            } else {
                braced!(content in content);
                let variable = Self::extract_variable(&content)?;
                html.children.push(Html {
                    children: vec![],
                    value: Lex::Var(variable),
                })
            }
        } else if input.peek(Token![if]) {
            input.parse::<Token![if]>()?;

            let condition = Self::extract_condition(input)?;

            let next_input;
            braced!(next_input in input);

            let next_input: ParseStream = &next_input;

            let mut new_html = Html {
                children: vec![],
                value: Lex::If(condition.to_string()),
            };

            Self::parse_html(&next_input, &mut new_html)?;

            html.children.push(new_html);
        } else if input.peek(Token![else]) {
            input.parse::<Token![else]>()?;

            let mut new_html = if input.peek(Token![if]) {
                input.parse::<Token![if]>()?;

                let condition = Self::extract_condition(input)?;
                Html {
                    children: vec![],
                    value: Lex::ElseIf(condition.to_string()),
                }
            } else {
                Html {
                    children: vec![],
                    value: Lex::Else,
                }
            };

            let next_input;
            braced!(next_input in input);

            let next_input: ParseStream = &next_input;

            Self::parse_html(&next_input, &mut new_html)?;

            html.children.push(new_html);
        } else if input.peek(Token![for]) {
            input.parse::<Token![for]>()?;

            let var: Ident = input.parse()?;

            input.parse::<Token![in]>()?;

            let iter: Ident = input.parse()?;

            let next_input;
            braced!(next_input in input);

            let mut new_html = Html {
                children: vec![],
                value: Lex::For {
                    iter: iter.to_string(),
                    var: var.to_string(),
                },
            };

            Self::parse_html(&&next_input, &mut new_html)?;

            html.children.push(new_html);
        } else {
            return Err(syn::Error::new(input.span(), "The token is invalid"));
        }

        Self::parse_html(input, html)?;

        Ok(html.clone())
    }

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

    fn get_next_input(input: &ParseStream, element: &HtmlElement) -> Result<Option<TokenStream>> {
        let mut opening_tags: Stack<HtmlElement> = Stack::new();

        let next_input = input.step(|cursor| {
            let result: Result<(Option<TokenStream>, Cursor)> = {
                let mut rest = *cursor;
                let mut tokens: Vec<TokenTree> = vec![];

                while let Some((tt, next)) = rest.token_tree() {
                    let punct = if let TokenTree::Punct(p) = &tt {
                        p
                    } else {
                        tokens.push(tt.clone());
                        rest = next;
                        continue;
                    };

                    if punct.as_char() != '<' {
                        tokens.push(tt.clone());
                        rest = next;
                        continue;
                    }

                    let (tt2, next2) = if let Some((tt2, next2)) = next.token_tree() {
                        (tt2, next2)
                    } else {
                        tokens.push(tt.clone());
                        rest = next;
                        continue;
                    };

                    if let TokenTree::Ident(i) = &tt2 {
                        if let Some(el) = HtmlElement::from_str(&i.to_string()) {
                            opening_tags.push(el.clone());
                        }
                    }

                    let punct = if let TokenTree::Punct(p) = &tt2 {
                        p
                    } else {
                        tokens.push(tt.clone());
                        tokens.push(tt2.clone());
                        rest = next2;
                        continue;
                    };

                    if punct.as_char() != '/' {
                        tokens.push(tt.clone());
                        tokens.push(tt2.clone());
                        rest = next2;
                        continue;
                    }

                    let (tt3, next3) = if let Some((tt3, next3)) = next2.token_tree() {
                        (tt3, next3)
                    } else {
                        tokens.push(tt.clone());
                        tokens.push(tt2.clone());
                        rest = next2;
                        continue;
                    };

                    let ident = if let TokenTree::Ident(i) = &tt3 {
                        i
                    } else {
                        tokens.push(tt.clone());
                        tokens.push(tt2.clone());
                        tokens.push(tt3.clone());
                        rest = next3;
                        continue;
                    };

                    if ident.to_string() != element.to_string() {
                        tokens.push(tt.clone());
                        tokens.push(tt2.clone());
                        tokens.push(tt3.clone());
                        rest = next3;
                        continue;
                    }

                    let (tt4, next4) = if let Some((tt4, next4)) = next3.token_tree() {
                        (tt4, next4)
                    } else {
                        tokens.push(tt.clone());
                        tokens.push(tt2.clone());
                        tokens.push(tt3.clone());
                        rest = next3;
                        continue;
                    };

                    let punct = if let TokenTree::Punct(p) = &tt4 {
                        p
                    } else {
                        tokens.push(tt.clone());
                        tokens.push(tt2.clone());
                        tokens.push(tt3.clone());
                        tokens.push(tt4.clone());
                        rest = next4;
                        continue;
                    };

                    if punct.as_char() != '>' {
                        tokens.push(tt.clone());
                        tokens.push(tt2.clone());
                        tokens.push(tt3.clone());
                        tokens.push(tt4.clone());
                        rest = next4;
                        continue;
                    }

                    if let Some(opening_tag) = opening_tags.peek() {
                        if opening_tag.to_string() == ident.to_string() {
                            opening_tags.pop();
                            tokens.push(tt.clone());
                            tokens.push(tt2.clone());
                            tokens.push(tt3.clone());
                            tokens.push(tt4.clone());
                            rest = next4;
                            continue;
                        }
                    }

                    if tokens.is_empty() {
                        return Ok((None, next4));
                    } else {
                        return Ok((Some(tokens.into_iter().collect()), next4));
                    }
                }

                if tokens.is_empty() {
                    Ok((None, rest))
                } else {
                    Ok((Some(tokens.into_iter().collect()), rest))
                }
            };

            return result;
        })?;

        Ok(next_input)
    }
}

#[derive(Debug, Clone)]
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
        if input.peek(Ident) {
            let key: Ident = input.parse()?;
            input.parse::<Token![=]>()?;

            if input.peek(Brace) {
                let mut value;
                braced!(value in input);
                braced!(value in value);
                let value: Expr = value.parse()?;
                return Ok(Attribute(key.to_string(), AttributeValue::Var(value)));
            }

            let value: LitStr = input.parse()?;
            return Ok(Attribute(
                key.to_string(),
                AttributeValue::Text(value.value()),
            ));
        }

        Err(syn::Error::new(
            input.span(),
            "Some attributes should be here.",
        ))
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
