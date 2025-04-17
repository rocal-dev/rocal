use rocal::rocal_core::traits::{SharedRouter, Template, View};

use crate::{
    templates::{sales_item_template::SalesItemTemplate, sales_log_template::SalesLogTemplate},
    view_models::{
        sales_item_view_model::SalesItemViewModel, sales_log_view_model::SalesLogViewModel,
    },
};

pub struct SalesView {
    router: SharedRouter,
}

impl View for SalesView {
    fn new(router: SharedRouter) -> Self {
        Self { router }
    }
}

impl SalesView {
    pub fn index(&self, view_model: SalesLogViewModel) {
        let template = SalesLogTemplate::new(self.router.clone());
        template.render(view_model);
    }

    pub fn show(&self, view_model: SalesItemViewModel) {
        let template = SalesItemTemplate::new(self.router.clone());
        template.render(view_model);
    }
}
