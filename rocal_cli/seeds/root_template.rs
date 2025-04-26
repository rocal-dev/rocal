use rocal::{
    rocal_core::traits::{SharedRouter, Template},
    view,
};

pub struct RootTemplate {
    router: SharedRouter,
}

impl Template for RootTemplate {
    type Data = String;

    fn new(router: SharedRouter) -> Self {
        RootTemplate { router }
    }

    fn body(&self, data: Self::Data) -> String {
        view! {
            <h1>{"Welcome to rocal world!"}</h1>
            <p>{{ &data }}</p>
        }
    }

    fn router(&self) -> SharedRouter {
        self.router.clone()
    }
}
