#[cfg(test)]
mod tests {
    //! Integration tests for the Html → TokenStream “tokenizer”.
    //!
    //! These tests
    //!   1. parse a DSL snippet into an `Html` AST with `parse()`
    //!   2. turn that AST into Rust tokens via `Tokenizer::to_token_stream()`
    //!   3. assert that the generated token text contains the expected bits
    //!      (opening/closing tags, attributes, flow-control keywords, …)

    use quote::quote;
    use rocal_ui::html::{parse, to_tokens::ToTokens};

    /// Convenience: parse and immediately stringify the generated tokens.
    fn gen(src: proc_macro2::TokenStream) -> String {
        let ast = parse(src).expect("parser should succeed");
        ast.to_token_stream().to_string()
    }

    #[test]
    fn simple_div_and_text() {
        let out = gen(quote! { <div>{ "Hi" }</div> });

        assert!(out.contains("let mut html"));
        assert!(out.contains("div"));
        assert!(out.contains("Hi"));
        assert!(out.contains("</div>"));
    }

    #[test]
    fn simple_button_and_text() {
        let out = gen(
            quote! { <button type="submit" class="w-full h-full cursor-pointer">{ "Submit" }</button> },
        );

        assert!(out.contains("Submit"));
    }

    #[test]
    fn void_tag_br_inside_paragraph() {
        let out = gen(quote! { <p>{ "Break" }<br />{ "next" }</p> });

        assert!(out.contains("p"));
        assert!(out.contains("br"));
        assert!(out.contains("next"));
        assert!(out.contains("</p>"));
    }

    #[test]
    fn attributes_render_correctly() {
        let out = gen(quote! { <div class="section" id={{main}}></div> });

        assert!(out.contains(r#""section""#));
        assert!(out.contains(r#"main"#));
        assert!(out.contains("</div>"));
    }

    #[test]
    fn nested_headers_and_paragraph() {
        let out = gen(quote! {
            <div class="section">
                <h1 class="title">{ "Hello, world!" }</h1>
                <h2 class="body">
                  <p id="item">{ "Hey, mate!" }</p>
                </h2>
            </div>
        });

        for needle in [
            r#"div"#,
            r#"class"#,
            r#""section""#,
            r#"h1"#,
            r#"class"#,
            r#""title""#,
            r#"h2"#,
            r#"class"#,
            r#""body""#,
            r#"p"#,
            r#"id"#,
            r#""item""#,
            "</div>",
            "</h2>",
            "</h1>",
        ] {
            assert!(
                out.contains(needle),
                "generated tokens should contain `{needle}`"
            );
        }
    }

    #[test]
    fn if_else_chain_in_html() {
        let out = gen(quote! {
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

        assert!(out.contains("if x == 1 || x == 2"));
        assert!(out.contains("else if x == 3"));
        assert!(out.contains("else {"));
        assert!(out.contains("span"));
    }

    #[test]
    fn variable_interpolation_emits_plain_ident() {
        let out = gen(quote! { <p>{{ name }}</p> });

        assert!(out.contains("push_str"));
        assert!(out.contains("(name)"));
    }

    #[test]
    fn for_loop_generates_rust_for() {
        let out = gen(quote! { for item in items { <li>{{ item }}</li> } });

        assert!(out.contains("for item in items"));
        assert!(out.contains("li"));
        assert!(out.contains("</li>"));
    }

    #[test]
    fn doc_type_declaration() {
        let out = gen(quote! { <!DOCTYPE html> });

        assert!(out.contains("<!DOCTYPE html>"));
    }

    #[test]
    fn async_and_defer_in_script_tag() {
        let out = gen(
            quote! { <script src="https://accounts.google.com/gsi/client" async defer></script> },
        );

        assert!(out.contains("async"));
        assert!(out.contains("defer"));
    }

    #[test]
    fn svg_image() {
        let out = gen(quote! {
            <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <path d="m21.64 3.64-1.28-1.28a1.21 1.21 0 0 0-1.72 0L2.36 18.64a1.21 1.21 0 0 0 0 1.72l1.28 1.28a1.2 1.2 0 0 0 1.72 0L21.64 5.36a1.2 1.2 0 0 0 0-1.72"/>
              <path d="m14 7 3 3"/>
              <path d="M5 6v4"/>
              <path d="M19 14v4"/>
              <path d="M10 2v2"/>
              <path d="M7 8H3"/>
              <path d="M21 16h-4"/>
              <path d="M11 3H9"/>
            </svg>
        });

        assert!(out.contains("svg"));
        assert!(out.contains("path"));
    }
}
