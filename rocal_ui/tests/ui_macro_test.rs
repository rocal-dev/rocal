#[cfg(test)]
mod tests {
    use quote::quote;
    use rocal_ui::build_ui;
    use rocal_ui::enums::html_element::HtmlElement;
    use rocal_ui::models::html_node::{Branch, Node};

    use crate::Item;

    fn test_build_ui() {
        let result = build_ui(quote! {
            <>
                <div class="container" id="main">
                  <h1>Test</h1>
                  <h2>[He{l}lo], wo(r)ld</h2>
                  {% if x == 1 %}
                    <p>x is one</p>
                  {% else if x == 2 %}
                    <p>x is two</p>
                  {% else %}
                    <p>x is not one nor two</p>
                  {% endif %}
                  {{ var }}
                </div>
                {% if !items.is_empty() %}
                  <ul>
                   {% for item in items %}
                     <li>{{ item.get_name() }}</li>
                   {% endfor %}
                 </ul>
                {% endif %}
            </>
        });

        assert_eq!(result.to_string(), String::new());
    }

    // #[test]
    fn test_build_ui_with_simple_html() {
        let result = build_ui(quote! {
            <div class="section" id="main">
                <h1 id="title">
                  <span id="label">{ "Header #1" }</span>
                </h1>
            </div>
        });

        println!("{:#?}", result);

        assert!(true);
    }

    // #[test]
    fn test_build_ui_with_node_having_multiple_children() {
        let result = build_ui(quote! {
            <div>
                <h1><span>{ "Header #1" }</span></h1>
                <h2>{ "Header #2" }</h2>
            </div>
        });

        println!("{:#?}", result);

        assert!(true);
    }

    // #[test]
    fn test_build_ui_with_node_having_void_tags() {
        let result = build_ui(quote! {
            <p>{ "Break" }<br />{ "this line!" }<br>{"And this"}</p>
        });

        println!("{:#?}", result);

        assert!(true);
    }

    // #[test]
    fn test_build_ui_with_node_having_attributes() {
        let result = build_ui(quote! {
            <div class="section" id="main">
                <h1 class="title">{ "Hello, world!" }</h1>
                <h2 class="body">
                  <p id="item">{ "Hey, mate!" }</p>
                </h2>
            </div>
        });

        println!("{:#?}", result);

        assert!(true);
    }

    // #[test]
    fn test_build_ui_with_node_having_nested_same_element() {
        let result = build_ui(quote! {
            <div>
                <div>{ "Hello" }</div>
                <div>{ "World" }</div>
            </div>
        });

        println!("{:#?}", result);

        assert!(true);
    }

    // #[test]
    fn test_build_ui_with_html_having_variables() {
        let result = build_ui(quote! {
            <h1>{"Hi, "}{{ greeting }}{"!"}</h1>
        });

        println!("{:#?}", result);

        assert!(true);
    }

    // #[test]
    fn test_build_ui_with_html_having_conditions() {
        let result = build_ui(quote! {
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

        println!("{:#?}", result);

        assert!(true);
    }

    #[test]
    fn test_build_ui_with_html_having_loop() {
        let result = build_ui(quote! {
            <div>
                <ul>
                  for item in items {
                    <li>{{ item.name }}</li>
                  }
                </ul>
            </div>
        });

        println!("{:#?}", result);

        assert!(true);
    }

    fn test_to_token_stream() {
        let node = Node::Element {
            element: HtmlElement::Fragment,
            attributes: vec![],
            children: vec![
                Node::Element {
                    element: HtmlElement::Div,
                    attributes: vec![(String::from("class"), String::from("section"))],
                    children: vec![
                        Node::Element {
                            element: HtmlElement::H1,
                            attributes: vec![],
                            children: vec![Node::Text(String::from("Hello, world"))],
                        },
                        Node::If {
                            branches: vec![
                                Branch::new(
                                    Some(String::from("x == 1")),
                                    Box::new(Node::Element {
                                        element: HtmlElement::P,
                                        attributes: vec![],
                                        children: vec![Node::Text(String::from("x is one"))],
                                    }),
                                ),
                                Branch::new(
                                    Some(String::from("x == 2")),
                                    Box::new(Node::Element {
                                        element: HtmlElement::P,
                                        attributes: vec![],
                                        children: vec![Node::Text(String::from("x is two"))],
                                    }),
                                ),
                                Branch::new(
                                    None,
                                    Box::new(Node::Element {
                                        element: HtmlElement::P,
                                        attributes: vec![],
                                        children: vec![Node::Text(String::from(
                                            "x is not one nor two",
                                        ))],
                                    }),
                                ),
                            ],
                        },
                        Node::Var(String::from("var")),
                    ],
                },
                Node::If {
                    branches: vec![Branch::new(
                        Some(String::from("!items.is_empty()")),
                        Box::new(Node::Element {
                            element: HtmlElement::Ul,
                            attributes: vec![],
                            children: vec![Node::For {
                                var: String::from("items"),
                                iter: String::from("item"),
                                body: Box::new(Node::Element {
                                    element: HtmlElement::Li,
                                    attributes: vec![],
                                    children: vec![Node::Expr(String::from("item.get_name()"))],
                                }),
                            }],
                        }),
                    )],
                },
            ],
        };

        let items: Vec<Item> = vec![];
        let x = 1;

        assert_eq!(node.to_token_stream(None).to_string(), quote!().to_string());
    }
}

struct Item {
    name: String,
}

impl Item {
    pub fn get_name(&self) -> &str {
        &self.name
    }
}
