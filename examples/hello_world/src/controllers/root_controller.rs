use crate::views::root_view::RootView;
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
    pub fn index(&self) {
        self.view.index();
    }
}
