use std::{collections::HashMap, future::Future, pin::Pin};

use regex::Regex;
use url::Url;

use crate::enums::request_method::RequestMethod;

type Action = Box<dyn Fn(HashMap<String, String>) -> Pin<Box<dyn Future<Output = ()>>>>;

struct Node {
    children: HashMap<String, Node>,
    action: Option<Action>,
}

pub struct Router {
    root: Node,
}

impl Router {
    const HOST: &str = "https://www.example.com";

    pub fn new() -> Self {
        Router {
            root: Node {
                children: HashMap::new(),
                action: None,
            },
        }
    }

    pub fn register(&mut self, method: RequestMethod, route: &str, action: Action) {
        let mut ptr = &mut self.root;

        if !ptr.children.contains_key(&method.to_string()) {
            ptr.children.insert(
                method.to_string(),
                Node {
                    children: HashMap::new(),
                    action: None,
                },
            );
        }

        ptr = ptr.children.get_mut(&method.to_string()).unwrap();

        for s in route.split("/") {
            if !ptr.children.contains_key(s) {
                ptr.children.insert(
                    s.to_string(),
                    Node {
                        children: HashMap::new(),
                        action: None,
                    },
                );
            }

            ptr = ptr.children.get_mut(s).unwrap();
        }

        ptr.action = Some(action);
    }

    pub async fn resolve(
        &self,
        method: RequestMethod,
        route: &str,
        action_args: Option<HashMap<String, String>>,
    ) -> bool {
        let mut route = route.to_string();
        let path_param_regex: Regex = Regex::new(r"^<(?<key>.+)>$").unwrap();

        let mut action_args: HashMap<String, String> = action_args.unwrap_or(HashMap::new());

        if let Ok(url) = Url::parse(&format!("{}{}", Self::HOST, route)) {
            for (k, v) in url.query_pairs() {
                action_args.insert(k.to_string(), v.to_string());
            }
            route = url.path().to_string();
        }

        let mut ptr = &self.root;

        if !ptr.children.contains_key(&method.to_string()) {
            return false;
        }

        ptr = ptr.children.get(&method.to_string()).unwrap();

        for s in route.split("/") {
            if !ptr.children.contains_key(s) {
                if let Some(param) = ptr
                    .children
                    .keys()
                    .find(|key| path_param_regex.is_match(key))
                {
                    let caps = path_param_regex.captures(&param).unwrap();
                    action_args.insert(caps["key"].to_string(), s.to_string());
                    ptr = ptr.children.get(param).unwrap();
                    continue;
                } else {
                    return false;
                }
            }

            ptr = ptr.children.get(s).unwrap();
        }

        if let Some(action) = &ptr.action {
            action(action_args).await;
            true
        } else {
            false
        }
    }
}
