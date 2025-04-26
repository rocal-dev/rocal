use crate::view_models::root_view_model::RootViewModel;
use rocal::{
    rocal_core::traits::{SharedRouter, Template},
    view,
};

pub struct RootTemplate {
    router: SharedRouter,
}

impl Template for RootTemplate {
    type Data = RootViewModel;

    fn new(router: SharedRouter) -> Self {
        RootTemplate { router }
    }

    fn body(&self, data: Self::Data) -> String {
        view! {
            <div class="bg-gray-800 text-white px-4 flex justify-between">
                <span><a href="#/">{ "Demo Self-Checkout" }</a></span>
                <span><a href="#/sales">{ "Logs" }</a></span>
            </div>

            <div class="flex items-start justify-center gap-12 p-10 bg-gray-100 min-h-screen">
                <div class="bg-gray-800 rounded-2xl p-6 shadow-lg w-[400px]">
                  <div class="grid grid-cols-3 gap-4 bg-gray-100 p-4 rounded-lg">
                  for product in data.get_products() {
                      <form action={{ format!("/carts/{}", product.get_id()) }} method="post" class="bg-white p-2 rounded-lg shadow text-center text-xs cursor-pointer">
                          <button type="submit" class="w-full h-full cursor-pointer">
                            <div class="font-semibold">{{ product.get_name()}}</div>
                            <div class="text-gray-500">{"$"}{{ &format!("{:.2}", product.get_price()) }}</div>
                          </button>
                      </form>
                  }
                  </div>

                  <div class="bg-white mt-6 p-4 rounded-lg text-sm shadow-inner space-y-1">
                  if data.get_cart_items().is_empty() {
                    { "-" }
                  } else {
                      for item in data.get_cart_items() {
                          <div class="flex justify-between">
                              <div class="flex justify-start gap-10">
                                <span>{{ item.get_product_name() }}</span>
                                <span>{{ &format!("x{}", item.get_number_of_items()) }}</span>
                                <span>{{ &format!("${:.2}", item.get_product_price()) }}</span>
                              </div>
                              <form action={{ &format!("/carts/{}", item.get_product_id()) }} class="block" method="delete">
                                <button type="submit" class="w-full h-full cursor-pointer">{"x"}</button>
                              </form>
                          </div>
                      }
                  }
                  </div>
                  <div class="bg-white mt-6 p-4 rounded-lg text-sm shadow-inner space-y-1">
                    <div class="flex justify-between">
                      <span>{ "Total" }</span>
                      <span>{{ &format!("${:.2}", data.get_total_price() )}}</span>
                    </div>
                  </div>
                  <form action="/sales/checkout">
                    <button class="mt-4 w-full bg-gray-300 text-gray-900 py-2 rounded-lg font-semibold cursor-pointer">{"Checkout"}</button>
                  </form>
                </div>
            </div>
        }
    }

    fn router(&self) -> SharedRouter {
        self.router.clone()
    }
}
