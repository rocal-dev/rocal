use std::cell::RefCell;
use std::rc::Rc;

use url::Url;
use web_sys::window;

use crate::enums::request_method::RequestMethod;
use crate::router::Router;

pub struct RouteHandler {
    router: Rc<RefCell<Router>>,
    not_found: Box<dyn Fn()>,
}

impl RouteHandler {
    pub fn new(router: Rc<RefCell<Router>>, not_found: Option<Box<dyn Fn()>>) -> Self {
        let not_found = match not_found {
            Some(nf) => nf,
            None => Box::new(Self::default_not_found_page),
        };

        RouteHandler { router, not_found }
    }

    pub async fn handle_route(&self) {
        let href = if let Some(w) = window() {
            if let Ok(href) = w.location().href() {
                href
            } else {
                (self.not_found)();
                return;
            }
        } else {
            (self.not_found)();
            return;
        };

        let url = match Url::parse(&href) {
            Ok(u) => u,
            Err(_) => {
                (self.not_found)();
                return;
            }
        };

        let path = url.fragment().unwrap_or_else(|| "/");

        if !self
            .router
            .borrow()
            .resolve(RequestMethod::Get, path, None)
            .await
        {
            (self.not_found)();
        }
    }

    fn default_not_found_page() {
        web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .body()
            .unwrap()
            .set_inner_html("<h1>404 - Page Not Found</h1>");
    }
}
