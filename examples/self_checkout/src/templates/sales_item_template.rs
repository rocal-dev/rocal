use rocal::rocal_core::traits::{SharedRouter, Template};

use crate::view_models::sales_item_view_model::SalesItemViewModel;

pub struct SalesItemTemplate {
    router: SharedRouter,
}

impl Template for SalesItemTemplate {
    type Data = SalesItemViewModel;

    fn new(router: SharedRouter) -> Self {
        Self { router }
    }

    fn body(&self, data: Self::Data) -> String {
        let mut html =
            String::from("<div class='bg-gray-800 text-white px-4 flex justify-between'><span><a href='#/'>Demo Self-Checkout</a></span><span><a href='#/sales'>Logs</a></span></div>");

        let mut items = String::from("<div>");

        for item in data.get_sales_items() {
            items += &format!(
                r#"<div class="flex justify-between"><span>{}</span><span>x{}</span><span>${:.2}</span></div>"#,
                item.get_product_name(),
                item.get_number_of_items(),
                item.get_product_price()
            );
        }

        items += &format!(
            r#"<div class="flex justify-between"><span>Total</span><span>${:.2}</span></div>"#,
            data.get_total_price()
        );

        items += "</div>";

        html += &format!(
            r#"
            <div class="flex items-start justify-center gap-12 p-10 bg-gray-100 min-h-screen">
              <div class="bg-gray-800 rounded-2xl p-6 shadow-lg w-[400px]">
                <div class="bg-gray-100 p-4 rounded-lg">{}</div>
              </div>
            </div>
            "#,
            items
        );

        html
    }

    fn router(&self) -> SharedRouter {
        self.router.clone()
    }
}
