use rocal::rocal_core::{
    enums::request_method::RequestMethod,
    traits::{Controller, SharedRouter},
};

use crate::{
    models::sales::Sales,
    repositories::{cart_repository::CartRepository, sales_repository::SalesRepository},
    view_models::{
        sales_item_view_model::SalesItemViewModel, sales_log_view_model::SalesLogViewModel,
    },
    views::sales_view::SalesView,
    CONFIG, FLASH_MEMORY,
};

pub struct SalesController {
    router: SharedRouter,
    view: SalesView,
}

impl Controller for SalesController {
    type View = SalesView;

    fn new(router: SharedRouter, view: Self::View) -> Self {
        Self { router, view }
    }
}

impl SalesController {
    #[rocal::action]
    pub async fn index(&self) {
        let sales_repo = SalesRepository::new(CONFIG.database.clone());

        let sales_logs = if let Ok(sales_logs) = sales_repo.get_all().await {
            sales_logs
        } else {
            vec![]
        };

        let vm = SalesLogViewModel::new(sales_logs);

        self.view.index(vm);
    }

    #[rocal::action]
    pub async fn show(&self, id: u32) {
        let sales_repo = SalesRepository::new(CONFIG.database.clone());

        let sales_items = if let Ok(items) = sales_repo.get_all_items(id).await {
            items
        } else {
            self.router
                .borrow()
                .resolve(RequestMethod::Get, "/sales", None)
                .await;
            return;
        };

        let vm = SalesItemViewModel::new(sales_items);

        self.view.show(vm);
    }

    #[rocal::action]
    pub async fn checkout(&self) {
        let sales_repo = SalesRepository::new(CONFIG.database.clone());
        let cart_repo = CartRepository::new(CONFIG.database.clone());

        let items = if let Ok(items) = cart_repo.get_all_items().await {
            items
                .into_iter()
                .map(|item| {
                    Sales::new(
                        *item.get_product_id(),
                        item.get_product_name(),
                        item.get_product_price(),
                        *item.get_number_of_items(),
                    )
                })
                .collect()
        } else {
            if let Ok(mut flash) = FLASH_MEMORY.lock() {
                let _ = flash.set("get_all_cart_items_error", "Could not get all cart items");
            }
            return;
        };

        if let Err(Some(err)) = sales_repo.create(items).await {
            if let Ok(mut flash) = FLASH_MEMORY.lock() {
                let _ = flash.set("sales_repo.create", &err);
            }
            return;
        }

        if let Err(Some(err)) = cart_repo.remove_all_items().await {
            if let Ok(mut flash) = FLASH_MEMORY.lock() {
                let _ = flash.set("cart_repo.remove_all_items", &err);
            }
            return;
        }

        self.router
            .borrow()
            .resolve(RequestMethod::Get, "/", None)
            .await;
    }
}
