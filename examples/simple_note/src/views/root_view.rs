use crate::{templates::root_template::RootTemplate, view_models::root_view_model::RootViewModel};
use rocal::rocal_core::traits::{SharedRouter, Template, View};
pub struct RootView {
    router: SharedRouter,
}
impl View for RootView {
    fn new(router: SharedRouter) -> Self {
        RootView { router }
    }
}
impl RootView {
    pub fn index(&self, vm: RootViewModel) {
        let template = RootTemplate::new(self.router.clone());
        template.render(vm);
    }
}
