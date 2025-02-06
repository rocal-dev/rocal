use std::{cell::RefCell, collections::HashMap, rc::Rc};
use url::Url;
use wasm_bindgen::{closure::Closure, JsCast};
use wasm_bindgen_futures::spawn_local;
use web_sys::{window, Document, Event, FormData, HtmlFormElement};

use crate::{enums::request_method::RequestMethod, router::Router};

pub type SharedRouter = Rc<RefCell<Router>>;

pub trait Controller {
    type View;
    fn new(router: SharedRouter, view: Self::View) -> Self;
}

pub trait View {
    fn new(router: SharedRouter) -> Self;
}

pub trait Template {
    type Data;

    fn new(router: SharedRouter) -> Self;
    fn router(&self) -> SharedRouter;
    fn body(&self, data: Self::Data) -> String;

    fn render(&self, data: Self::Data) {
        self.render_html(&self.body(data));
        self.register_forms();
    }

    fn render_html(&self, html: &str) {
        let doc = match self.get_document() {
            Some(doc) => doc,
            None => return,
        };

        let body = match doc.body() {
            Some(body) => body,
            None => return,
        };

        body.set_inner_html(html);
    }

    fn register_forms(&self) {
        let doc = match self.get_document() {
            Some(doc) => doc,
            None => return,
        };

        let forms = match self.get_all_forms(&doc) {
            Some(forms) => forms,
            None => return,
        };

        for i in 0..forms.length() {
            if let Some(form_node) = forms.get(i) {
                if let Some(form) = self.reset_form(form_node) {
                    self.attach_form_listener(&form);
                }
            }
        }
    }

    fn get_document(&self) -> Option<Document> {
        window()?.document()
    }

    fn get_all_forms(&self, doc: &Document) -> Option<web_sys::NodeList> {
        doc.query_selector_all("form").ok()
    }

    fn reset_form(&self, form_node: web_sys::Node) -> Option<HtmlFormElement> {
        let parent = form_node.parent_node()?;
        let new_node = form_node.clone_node_with_deep(true).ok()?;
        parent.replace_child(&new_node, &form_node).ok()?;
        new_node.dyn_into::<HtmlFormElement>().ok()
    }

    fn attach_form_listener(&self, form: &HtmlFormElement) {
        let router_for_closure = self.router().clone();

        let closure = Closure::wrap(Box::new(move |e: Event| {
            e.prevent_default();

            let mut args: HashMap<String, String> = HashMap::new();

            let element: HtmlFormElement = match e
                .current_target()
                .and_then(|t| t.dyn_into::<HtmlFormElement>().ok())
            {
                Some(el) => el,
                None => return,
            };

            let form_data = match FormData::new_with_form(&element) {
                Ok(data) => data,
                Err(_) => return,
            };

            let entries = form_data.entries();

            for entry in entries {
                if let Ok(entry) = entry {
                    let entry_array = js_sys::Array::from(&entry);
                    if entry_array.length() == 2 {
                        let key = entry_array.get(0).as_string().unwrap_or_default();
                        let value = entry_array.get(1).as_string().unwrap_or_default();
                        args.insert(key, value);
                    }
                }
            }

            if let Ok(url) = Url::parse(&element.action()) {
                let router = router_for_closure.clone();
                spawn_local(async move {
                    router
                        .borrow()
                        .resolve(RequestMethod::Post, url.path(), Some(args))
                        .await;
                });
            }
        }) as Box<dyn FnMut(Event)>);

        form.add_event_listener_with_callback("submit", closure.as_ref().unchecked_ref())
            .expect("Failed to add submit event listeners");
        closure.forget();
    }
}
