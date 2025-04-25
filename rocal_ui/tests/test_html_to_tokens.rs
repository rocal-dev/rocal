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
    use rocal_ui::html5::{parse, to_tokens::ToTokens};

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

        assert!(out.contains("html += name"));
    }

    #[test]
    fn for_loop_generates_rust_for() {
        let out = gen(quote! { for item in items { <li>{{ item }}</li> } });

        assert!(out.contains("for item in items"));
        assert!(out.contains("li"));
        assert!(out.contains("</li>"));
    }
}
