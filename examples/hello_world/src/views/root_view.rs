use crate::templates::root_template::RootTemplate;
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
    pub fn index(&self) {
        let template = RootTemplate::new(self.router.clone());
        template.render(String::new());
    }
}
