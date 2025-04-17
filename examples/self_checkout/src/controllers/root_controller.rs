use crate::{
    repositories::{cart_repository::CartRepository, product_repository::ProductRepository},
    view_models::root_view_model::RootViewModel,
    views::root_view::RootView,
    CONFIG,
};
use rocal::rocal_core::traits::{Controller, SharedRouter};

pub struct RootController {
    router: SharedRouter,
    view: RootView,
}

impl Controller for RootController {
    type View = RootView;
    fn new(router: SharedRouter, view: Self::View) -> Self {
        RootController { router, view }
    }
}

impl RootController {
    #[rocal::action]
    pub async fn index(&self) {
        let product_repo = ProductRepository::new(CONFIG.database.clone());
        let cart_repo = CartRepository::new(CONFIG.database.clone());

        let products = if let Ok(products) = product_repo.get_all().await {
            products
        } else {
            vec![]
        };

        let cart_items = if let Ok(cart_items) = cart_repo.get_all_items().await {
            cart_items
        } else {
            vec![]
        };

        let vm = RootViewModel::new(products, cart_items);

        self.view.index(vm);
    }
}
