use crate::view_models::root_view_model::RootViewModel;
use rocal::rocal_core::traits::{SharedRouter, Template};

pub struct RootTemplate {
    router: SharedRouter,
}

impl Template for RootTemplate {
    type Data = RootViewModel;

    fn new(router: SharedRouter) -> Self {
        RootTemplate { router }
    }

    fn body(&self, data: Self::Data) -> String {
        let mut html =
            String::from("<div class='bg-gray-800 text-white px-4 flex justify-between'><span><a href='#/'>Demo Self-Checkout</a></span><span><a href='#/sales'>Logs</a></span></div>");

        html += r#"
<div class="flex items-start justify-center gap-12 p-10 bg-gray-100 min-h-screen">
  <div class="bg-gray-800 rounded-2xl p-6 shadow-lg w-[400px]">
            "#;

        html += r#"<div class="grid grid-cols-3 gap-4 bg-gray-100 p-4 rounded-lg">"#;

        for product in data.get_products() {
            html += &format!(
                r#"
<form action="/carts/{}" method="post" class="bg-white p-2 rounded-lg shadow text-center text-xs cursor-pointer">
  <button type="submit" class="w-full h-full cursor-pointer">
  <!--  <img src="/img/apple.png" class="w-10 h-10 mx-auto mb-1" /> -->
    <div class="font-semibold">{}</div>
    <div class="text-gray-500">${}</div>
  </button>
</form>
"#,
                product.get_id(),
                product.get_name(),
                product.get_price()
            );
        }

        html += "</div>";

        html += r#"<div class="bg-white mt-6 p-4 rounded-lg text-sm shadow-inner space-y-1">"#;

        let items = data.get_cart_items();

        if items.is_empty() {
            html += "-";
        } else {
            for item in items {
                html += &format!(
                    r#"
<div class="flex justify-between">
  <div class="flex justify-start gap-10">
    <span>{}</span><span>x{}</span><span>${:.2}</span>
  </div>
  <form action="/carts/{}" class="block" method="delete">
    <button type="submit" class="w-full h-full cursor-pointer">x</button>
  </form>
</div>
"#,
                    item.get_product_name(),
                    item.get_number_of_items(),
                    item.get_product_price(),
                    item.get_product_id()
                );
            }
        }

        html += "</div>";

        html += r#"<div class="bg-white mt-6 p-4 rounded-lg text-sm shadow-inner space-y-1">"#;

        html += &format!(
            r#"
<div class="flex justify-between">
  <span>Total</span><span>${:.2}</span>
</div>
"#,
            data.get_total_price()
        );

        html += "</div>";

        html += r#"
<form action="/sales/checkout">
  <button class="mt-4 w-full bg-gray-300 text-gray-900 py-2 rounded-lg font-semibold cursor-pointer">
    Checkout
  </button>
</form>
"#;
        html += "</div>";
        html += "</div>";

        html
    }

    fn router(&self) -> SharedRouter {
        self.router.clone()
    }
}
