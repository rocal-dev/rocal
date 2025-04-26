use rocal::{
    rocal_core::traits::{SharedRouter, Template},
    view,
};

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
        view! {
            <div class="bg-gray-800 text-white px-4 flex justify-between">
                <span><a href="#/">{"Demo Self-Checkout"}</a></span>
                <span><a href="#/sales">{"Logs"}</a></span>
            </div>
            <div class="flex items-start justify-center gap-12 p-10 bg-gray-100 min-h-screen">
              <div class="bg-gray-800 rounded-2xl p-6 shadow-lg w-[400px]">
                <div class="bg-gray-100 p-4 rounded-lg">
                  <div>
                  for item in data.get_sales_items() {
                      <div class="flex justify-between">
                          <span>{{ item.get_product_name()  }}</span>
                          <span>{{ &format!("x{}", item.get_number_of_items())  }}</span>
                          <span>{{ &format!("${:.2}", item.get_product_price()) }}</span>
                      </div>
                  }
                    <div class="flex justify-between">
                      <span>{"Total"}</span>
                      <span>{{ &format!("${:.2}", data.get_total_price()) }}</span>
                    </div>
                  </div>
                </div>
              </div>
            </div>
        }
    }

    fn router(&self) -> SharedRouter {
        self.router.clone()
    }
}
