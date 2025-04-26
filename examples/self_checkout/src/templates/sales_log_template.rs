use rocal::{
    rocal_core::traits::{SharedRouter, Template},
    view,
};

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
        view! {
            <div class="bg-gray-800 text-white px-4 flex justify-between">
                <span><a href="#/">{"Demo Self-Checkout"}</a></span>
                <span><a href="#/sales">{"Logs"}</a></span>
            </div>
            <div class="flex items-start justify-center gap-12 p-10 bg-gray-100 min-h-screen">
                <div class="bg-gray-800 rounded-2xl p-6 shadow-lg w-[400px]">
                  <div class="bg-gray-100 p-4 rounded-lg">
                    <ul>
                    for log in data.get_sales_logs() {
                      <li>
                        <a href={{ &format!("#/sales/{}", log.get_id()) }} class="underline hover:text-blue-600 dark:hover:text-blue-400">{{ log.get_created_at()  }}</a>
                      </li>
                    }
                    </ul>
                  </div>
                </div>
            </div>
        }
    }

    fn router(&self) -> SharedRouter {
        self.router.clone()
    }
}
