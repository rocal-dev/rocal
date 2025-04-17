use core::fmt;

#[derive(Debug)]
pub enum RequestMethod {
    Get,
    Post,
    Put,
    Patch,
    Delete,
}

impl RequestMethod {
    pub fn from(method: &str) -> Self {
        match method.to_uppercase().as_str() {
            "GET" => RequestMethod::Get,
            "POST" => RequestMethod::Post,
            "PUT" => RequestMethod::Put,
            "PATCH" => RequestMethod::Patch,
            "DELETE" => RequestMethod::Delete,
            _ => RequestMethod::Post,
        }
    }
}

impl fmt::Display for RequestMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RequestMethod::Get => write!(f, "GET"),
            RequestMethod::Post => write!(f, "POST"),
            RequestMethod::Put => write!(f, "PUT"),
            RequestMethod::Patch => write!(f, "PATCH"),
            RequestMethod::Delete => write!(f, "DELETE"),
        }
    }
}
