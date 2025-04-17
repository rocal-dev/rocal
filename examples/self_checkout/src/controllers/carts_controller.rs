use crate::{
    repositories::cart_repository::CartRepository, views::empty_view::EmptyView, CONFIG,
    FLASH_MEMORY,
};
use rocal::rocal_core::{
    enums::request_method::RequestMethod,
    traits::{Controller, SharedRouter},
};

pub struct CartsController {
    router: SharedRouter,
    view: EmptyView,
}

impl Controller for CartsController {
    type View = EmptyView;

    fn new(router: SharedRouter, view: Self::View) -> Self {
        Self { router, view }
    }
}

impl CartsController {
    #[rocal::action]
    pub async fn add(&self, product_id: u32) {
        let cart_repo = CartRepository::new(CONFIG.database.clone());

        if let Err(Some(err)) = cart_repo.add_item(product_id).await {
            if let Ok(mut flash) = FLASH_MEMORY.lock() {
                let _ = flash.set("add_item_to_cart_error", &err);
            }
            return;
        }

        self.router
            .borrow()
            .resolve(RequestMethod::Get, "/", None)
            .await;
    }

    #[rocal::action]
    pub async fn delete(&self, product_id: u32) {
        let cart_repo = CartRepository::new(CONFIG.database.clone());

        if let Err(Some(err)) = cart_repo.remove_item(product_id).await {
            if let Ok(mut flash) = FLASH_MEMORY.lock() {
                let _ = flash.set("delete_item_from_cart_error", &err);
            }
            return;
        }

        self.router
            .borrow()
            .resolve(RequestMethod::Get, "/", None)
            .await;
    }
}
