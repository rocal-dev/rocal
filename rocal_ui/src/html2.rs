use crate::{data_types::stack::Stack, enums::html_element::HtmlElement};
use proc_macro2::{Delimiter, TokenStream, TokenTree};
use syn::{
    braced,
    buffer::Cursor,
    parse::{Parse, ParseBuffer, ParseStream, Parser, Result},
    token::Brace,
    Ident, LitStr, Token,
};

#[derive(Default, Clone, Debug)]
pub struct Html2 {
    children: Vec<Html2>,
    value: Option<Lex>,
}

impl Parse for Html2 {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut root = Html2 {
            children: vec![],
            value: Some(Lex::Tag {
                element: HtmlElement::Fragment,
                attributes: vec![],
            }),
        };
        Ok(parse_html2(&input, &mut root)?)
    }
}

fn parse_html2(input: &ParseStream, html: &mut Html2) -> Result<Html2> {
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
                html.children.push(Html2 {
                    children: vec![],
                    value: Some(Lex::Tag {
                        element,
                        attributes: attrs,
                    }),
                });
            } else {
                let next_input = get_next_input(&input, &element)?;

                let mut new_html = Html2 {
                    children: vec![],
                    value: Some(Lex::Tag {
                        element,
                        attributes: attrs,
                    }),
                };

                if let Some(next_input) = next_input {
                    let parser = |input: ParseStream| parse_html2(&input, &mut new_html);
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
            html.children.push(Html2 {
                children: vec![],
                value: Some(Lex::Text(content.value())),
            });
        } else {
            braced!(content in content);
            let variable = extract_variable(&content)?;
            html.children.push(Html2 {
                children: vec![],
                value: Some(Lex::Var(variable)),
            })
        }
    } else if input.peek(Token![if]) {
        input.parse::<Token![if]>()?;

        let condition = extract_condition(input)?;

        let next_input;
        braced!(next_input in input);

        let next_input: ParseStream = &next_input;

        let mut new_html = Html2 {
            children: vec![],
            value: Some(Lex::If(condition.to_string())),
        };

        parse_html2(&next_input, &mut new_html)?;

        html.children.push(new_html);
    } else if input.peek(Token![else]) {
        input.parse::<Token![else]>()?;

        let mut new_html = if input.peek(Token![if]) {
            input.parse::<Token![if]>()?;

            let condition = extract_condition(input)?;
            Html2 {
                children: vec![],
                value: Some(Lex::ElseIf(condition.to_string())),
            }
        } else {
            Html2 {
                children: vec![],
                value: Some(Lex::Else),
            }
        };

        let next_input;
        braced!(next_input in input);

        let next_input: ParseStream = &next_input;

        parse_html2(&next_input, &mut new_html)?;

        html.children.push(new_html);
    } else if input.peek(Token![for]) {
        input.parse::<Token![for]>()?;

        let iter: Ident = input.parse()?;

        input.parse::<Token![in]>()?;

        let var: Ident = input.parse()?;

        let next_input;
        braced!(next_input in input);

        let mut new_html = Html2 {
            children: vec![],
            value: Some(Lex::For {
                iter: iter.to_string(),
                var: var.to_string(),
            }),
        };

        parse_html2(&&next_input, &mut new_html)?;

        html.children.push(new_html);
    } else {
        return Err(syn::Error::new(input.span(), "The token is invalid"));
    }

    parse_html2(input, html)?;

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
                    return Ok((None, rest));
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

#[derive(Debug, Clone)]
enum Lex {
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
struct Attribute((String, String));

impl Parse for Attribute {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(Ident) {
            let key: Ident = input.parse()?;
            input.parse::<Token![=]>()?;
            let value: LitStr = input.parse()?;

            return Ok(Attribute((key.to_string(), value.value())));
        }

        Err(syn::Error::new(
            input.span(),
            "Some attributes should be here.",
        ))
    }
}

// <h1><span>{"1"}</span></h1>
// <h2><span>{"2"}</span></h2>

// html: { children: [h1(<span>{"1"}</span>), h2(<span>{"2"}</span>)] }
// html: { children: [h1 { children: [span({"1"})]}, h2 { children: [span({"2"})]}] }
// html: { children: [h1 { children: [span {children: [text(1)]]}}, h2 { children: [span { children: [text(2)]}]}] }
