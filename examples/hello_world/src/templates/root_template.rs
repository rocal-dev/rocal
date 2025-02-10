use rocal::rocal_core::traits::{SharedRouter, Template};
pub struct RootTemplate {
    router: SharedRouter,
}
impl Template for RootTemplate {
    type Data = String;
    fn new(router: SharedRouter) -> Self {
        RootTemplate { router }
    }
    fn body(&self, data: Self::Data) -> String {
        let mut html = String::from("<h1>Welcome to rocal world!</h1>");
        html += &format!("<p>{}</p>", &data);
        html += &format!("<p><a href='#/sync-connections'>Sync settings</a><p>");
        html
    }
    fn router(&self) -> SharedRouter {
        self.router.clone()
    }
}
