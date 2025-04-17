use proc_macro2::TokenStream;
use syn::{
    braced,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Ident, LitStr, Token,
};

use crate::enums::request_method::RequestMethod;

mod kw {
    syn::custom_keyword!(get);
    syn::custom_keyword!(post);
    syn::custom_keyword!(put);
    syn::custom_keyword!(patch);
    syn::custom_keyword!(delete);
    syn::custom_keyword!(controller);
    syn::custom_keyword!(action);
    syn::custom_keyword!(view);
}

pub fn parse_routes(item: TokenStream) -> Result<Vec<ParsedRoute>, syn::Error> {
    let routes: ParsedRoutes = syn::parse(item.into())?;

    Ok(routes.0)
}

#[derive(Debug)]
pub struct ParsedRoutes(Vec<ParsedRoute>);

impl Parse for ParsedRoutes {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let routes = Punctuated::<ParsedRoute, Token!(,)>::parse_terminated(&input)?;
        let mut result: Vec<ParsedRoute> = vec![];
        routes.into_iter().for_each(|route| {
            result.push(route);
        });
        Ok(ParsedRoutes(result))
    }
}

#[derive(Debug, Default)]
pub struct ParsedRoute {
    method: Option<RequestMethod>,
    path: Option<String>,
    controller: Option<Ident>,
    action: Option<Ident>,
    view: Option<Ident>,
}

impl ParsedRoute {
    pub fn set_method(&mut self, method: RequestMethod) {
        self.method = Some(method);
    }

    pub fn set_path(&mut self, path: String) {
        self.path = Some(path);
    }

    pub fn set_controller(&mut self, controller: Ident) {
        self.controller = Some(controller);
    }

    pub fn set_action(&mut self, action: Ident) {
        self.action = Some(action);
    }

    pub fn set_view(&mut self, view: Ident) {
        self.view = Some(view);
    }

    pub fn get_method(&self) -> &Option<RequestMethod> {
        &self.method
    }

    pub fn get_path(&self) -> &Option<String> {
        &self.path
    }

    pub fn get_controller(&self) -> &Option<Ident> {
        &self.controller
    }

    pub fn get_action(&self) -> &Option<Ident> {
        &self.action
    }

    pub fn get_view(&self) -> &Option<Ident> {
        &self.view
    }
}

impl Parse for ParsedRoute {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let mut route = ParsedRoute::default();

        let method = if input.peek(kw::get) {
            input
                .parse::<kw::get>()
                .expect("we just checked for this token");
            RequestMethod::Get
        } else if input.peek(kw::post) {
            input
                .parse::<kw::post>()
                .expect("we just checked for this token");
            RequestMethod::Post
        } else if input.peek(kw::put) {
            input
                .parse::<kw::put>()
                .expect("we just checked for this token");
            RequestMethod::Put
        } else if input.peek(kw::patch) {
            input
                .parse::<kw::patch>()
                .expect("we just checked for this token");
            RequestMethod::Patch
        } else if input.peek(kw::delete) {
            input
                .parse::<kw::delete>()
                .expect("we just checked for this token");
            RequestMethod::Delete
        } else {
            return Err(syn::Error::new(
                input.span(),
                "Method should be get, post, put, patch, or delete",
            ));
        };

        route.set_method(method);

        let path = input
            .parse()
            .map(|v: LitStr| v.value())
            .map_err(|_| syn::Error::new(input.span(), "Path is required"))?;

        let _: Token!(=>) = input.parse().map_err(|_| {
            syn::Error::new(
                input.span(),
                "Path and destination should be separated by =>",
            )
        })?;

        route.set_path(path);

        let dst;
        braced!(dst in input);

        let kvs = Punctuated::<KeyValue, Token!(,)>::parse_terminated(&dst)?;

        kvs.into_iter().for_each(|kv| {
            if kv.key == "controller" {
                route.set_controller(kv.value);
            } else if kv.key == "action" {
                route.set_action(kv.value);
            } else if kv.key == "view" {
                route.set_view(kv.value);
            }
        });

        Ok(route)
    }
}

#[derive(Debug)]
struct KeyValue {
    key: String,
    value: Ident,
}

impl Parse for KeyValue {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let key = input.parse().map(|v: Ident| v.to_string()).map_err(|_| {
            syn::Error::new(
                input.span(),
                "should have property keys within curly braces",
            )
        })?;

        let _: Token!(:) = input.parse().map_err(|_| {
            syn::Error::new(input.span(), "prop key and value should be separated by :")
        })?;

        let value: Ident = if key == "controller" || key == "action" || key == "view" {
            input
                .parse()
                .map_err(|_| syn::Error::new(input.span(), "Property requires a value"))
        } else {
            Err(syn::Error::new(
                input.span(),
                format!("unknown property key: {}", key),
            ))
        }?;

        Ok(KeyValue { key, value })
    }
}
