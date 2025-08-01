#[cfg(test)]
mod tests {
    use quote::{quote, ToTokens};
    use rocal_ui::{
        enums::html_element::HtmlElement,
        html::{parse, AttributeValue, Html, Lex},
    };

    // ---------- helpers ----------
    /// Return the first (and only) child as a convenience.
    fn only_child(root: &Html) -> &Html {
        root.children()
            .get(0)
            .expect("root should have exactly one child")
    }

    /// Shorthand for `parse()` that panics on error.
    fn parse_ok(ts: proc_macro2::TokenStream) -> Html {
        parse(ts).expect("parser must succeed")
    }

    // ---------- success cases ----------

    #[test]
    fn parses_single_element() {
        let html = parse_ok(quote! { <div></div> });

        let div = only_child(&html);
        if let Lex::Tag { element, .. } = &div.value() {
            assert_eq!(*element, HtmlElement::Div);
        } else {
            panic!("expected <div> tag");
        }
    }

    #[test]
    fn parses_void_element() {
        let html = parse_ok(quote! { <br/> });

        let br = only_child(&html);
        assert!(
            matches!(
                br.value(),
                Lex::Tag {
                    element: HtmlElement::Br,
                    ..
                }
            ),
            "expected <br> tag"
        );
    }

    #[test]
    fn parses_attributes() {
        let html = parse_ok(quote! { <img src="logo.png" alt="Logo"/> });

        let img = only_child(&html);
        if let Lex::Tag { attributes, .. } = &img.value() {
            let attrs: Vec<_> = attributes
                .iter()
                .map(|attr| {
                    let value = match attr.value() {
                        Some(AttributeValue::Text(text)) => text,
                        Some(AttributeValue::Var(var)) => &var.to_token_stream().to_string(),
                        None => "",
                    };
                    (attr.key(), value.to_string())
                })
                .collect();

            assert!(attrs.contains(&("src", "logo.png".to_string())));
            assert!(attrs.contains(&("alt", "Logo".to_string())));
        } else {
            panic!("expected <img> tag");
        }
    }

    #[test]
    fn parses_attributes_including_variable() {
        let html = parse_ok(quote! { <a href={{ url }}>{{ url }}</a> });

        let a = only_child(&html);
        if let Lex::Tag { attributes, .. } = &a.value() {
            let attrs: Vec<_> = attributes
                .iter()
                .map(|attr| {
                    let value = match attr.value() {
                        Some(AttributeValue::Text(text)) => text,
                        Some(AttributeValue::Var(var)) => &var.to_token_stream().to_string(),
                        None => "",
                    };
                    (attr.key(), value.to_string())
                })
                .collect();

            assert!(attrs.contains(&("href", "url".to_string())));
        } else {
            panic!("expected <a> tag");
        }
    }

    #[test]
    fn parses_nested_elements() {
        let html = parse_ok(quote! { <div><span></span></div> });

        let div = only_child(&html);
        let span = only_child(div);
        assert!(
            matches!(
                span.value(),
                Lex::Tag {
                    element: HtmlElement::Span,
                    ..
                }
            ),
            "expected nested <span>"
        );
    }

    #[test]
    fn parses_text_and_variable() {
        // { "Hello" }  -> text
        // {{ name }}   -> variable
        let html = parse_ok(quote! { { "Hello" } {{ name }} });

        assert_eq!(html.children().len(), 2);

        matches!(
            html.children().get(0).unwrap().value(),
            Lex::SanitizedVar(ref s) if s == "Hello"
        );
        matches!(html.children().get(1).unwrap().value(), Lex::Var(_));
    }

    #[test]
    fn parses_if_else_chain() {
        let html = parse_ok(quote! {
            if x == 1 { <p></p> } else if x == 2 { <p></p> } else { <p></p> }
        });

        // fragment -> [If, ElseIf, Else]
        assert_eq!(html.children().len(), 3);

        assert!(matches!(
            html.children().get(0).unwrap().value(),
            Lex::If(_)
        ));
        assert!(matches!(
            html.children().get(1).unwrap().value(),
            Lex::ElseIf(_)
        ));
        assert!(matches!(html.children().get(2).unwrap().value(), Lex::Else));
    }

    #[test]
    fn parses_for_loop() {
        let html = parse_ok(quote! { for item in items { <li></li> } });

        let for_node = only_child(&html);
        if let Lex::For { iter, var } = &for_node.value() {
            assert_eq!(iter, "items");
            assert_eq!(var, "item");
        } else {
            panic!("expected for-loop node");
        }
    }

    // ------------------------------------------------------------
    // <div class="section" id="main">…<h1>…<span>…</span></h1></div>
    // ------------------------------------------------------------
    #[test]
    fn parses_section_header() {
        let html = parse_ok(quote! {
            <div class="section" id="main">
                <h1 id="title">
                  <span id="label">{ "Header #1" }</span>
                </h1>
            </div>
        });

        assert!(
            matches!(
                html.children().get(0).unwrap().value(),
                Lex::Tag {
                    element: HtmlElement::Div,
                    ..
                }
            ),
            "root child should be <div>"
        );
    }

    // ------------------------------------------------------------
    // <p>{ "Break" }<br />{ "this line!" }<br>{"And this"}</p>
    // ------------------------------------------------------------
    #[test]
    fn parses_break_and_text_lines() {
        let html = parse_ok(quote! {
            <p>{ "Break" }<br />{ "this line!" }<br>{"And this"}</p>
        });

        assert!(
            matches!(
                html.children().get(0).unwrap().value(),
                Lex::Tag {
                    element: HtmlElement::P,
                    ..
                }
            ),
            "root child should be <p>"
        );
    }

    // ------------------------------------------------------------
    // second big <div> with two headers and nested <p>
    // ------------------------------------------------------------
    #[test]
    fn parses_nested_headers() {
        let html = parse_ok(quote! {
            <div class="section" id="main">
                <h1 class="title">{ "Hello, world!" }</h1>
                <h2 class="body">
                  <p id="item">{ "Hey, mate!" }</p>
                </h2>
            </div>
        });

        assert!(matches!(
            html.children().get(0).unwrap().value(),
            Lex::Tag {
                element: HtmlElement::Div,
                ..
            }
        ));
    }

    // ------------------------------------------------------------
    //   <div> <div>{ "Hello" }</div> <div>{ "World" }</div> </div>
    // ------------------------------------------------------------
    #[test]
    fn parses_hello_world_divs() {
        let html = parse_ok(quote! {
            <div>
                <div>{ "Hello" }</div>
                <div>{ "World" }</div>
            </div>
        });

        assert!(matches!(
            html.children().get(0).unwrap().value(),
            Lex::Tag {
                element: HtmlElement::Div,
                ..
            }
        ));
    }

    // ------------------------------------------------------------
    // if / else-if / else chain inside a <div>
    // ------------------------------------------------------------
    #[test]
    fn parses_div_with_if_else_chain() {
        let html = parse_ok(quote! {
            <div>
                if x == 1 || x == 2 {
                    <span>{ "x is 1 or 2" }</span>
                } else if x == 3 {
                    <span>{ "x is 3" }</span>
                } else {
                    if y == 1 {
                        <span>{ "y is 1 but x is unknown" }</span>
                    } else {
                        <span>{ "x and y are unknown" }</span>
                    }
                }
            </div>
        });

        // div is first, then the `if` node is its first child
        let div = &html.children().get(0).unwrap();
        assert!(matches!(
            div.value(),
            Lex::Tag {
                element: HtmlElement::Div,
                ..
            }
        ));
        assert!(matches!(div.children().get(0).unwrap().value(), Lex::If(_)));
    }

    #[test]
    fn can_parse_doctype() {
        let result = parse(quote! {
            <!DOCTYPE html>
        });
        assert!(result.is_ok());
    }

    #[test]
    fn parse_attributes_including_hyphen_separeted_keys() {
        let result = parse(quote! {
            <meta http-equiv="X-UA-Compatible">
        });
        assert!(result.is_ok());
    }

    #[test]
    fn parse_async_and_defer_in_script_tag() {
        let result = parse(quote! {
            <script src="https://accounts.google.com/gsi/client" async defer></script>
        });
        assert!(result.is_ok());
    }

    #[test]
    fn parse_attribute_including_type_following_by_dash() {
        let result = parse(quote! {
            <div data-type="type"></div>
        });
        assert!(result.is_ok());
    }

    #[test]
    fn parse_label() {
        let result = parse(quote! {
          <label for="label" class="block text-sm font-medium text-gray-700">
            {"Label"}
          </label>
        });
        assert!(result.is_ok());
    }

    #[test]
    fn parse_svg() {
        let result = parse(quote! {
            <svg class="w-8 h-8 text-green-600" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" d="M5 13l4 4L19 7" />
            </svg>
        });
        assert!(result.is_ok());
    }

    // ---------- failure cases ----------

    #[test]
    fn fails_on_garbage_token() {
        let result = parse(quote! { ??? });
        assert!(result.is_err());
    }

    #[test]
    fn fails_on_unclosed_tag() {
        let result = parse(quote! {
            <div>
              <input
            </div>
        });
        assert!(result.is_err());
    }
}
