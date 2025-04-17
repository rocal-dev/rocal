use rocal::rocal_core::traits::{SharedRouter, Template};

use crate::view_models::sales_log_view_model::SalesLogViewModel;

pub struct SalesLogTemplate {
    router: SharedRouter,
}

impl Template for SalesLogTemplate {
    type Data = SalesLogViewModel;

    fn new(router: SharedRouter) -> Self {
        Self { router }
    }

    fn body(&self, data: Self::Data) -> String {
        let mut logs = String::from("<ul>");

        for log in data.get_sales_logs() {
            logs += &format!(
                r##"
                <li><a href="#/sales/{}" class="underline hover:text-blue-600 dark:hover:text-blue-400">{}</a></li>
                "##,
                log.get_id(),
                log.get_created_at()
            );
        }

        logs += "</ul>";

        let mut html =
            String::from("<div class='bg-gray-800 text-white px-4 flex justify-between'><span><a href='#/'>Demo Self-Checkout</a></span><span><a href='#/sales'>Logs</a></span></div>");

        html += &format!(
            r#"
            <div class="flex items-start justify-center gap-12 p-10 bg-gray-100 min-h-screen">
              <div class="bg-gray-800 rounded-2xl p-6 shadow-lg w-[400px]">
                <div class="bg-gray-100 p-4 rounded-lg">{}</div>
              </div>
            </div>
            "#,
            logs
        );

        html
    }

    fn router(&self) -> SharedRouter {
        self.router.clone()
    }
}
