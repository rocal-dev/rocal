use core::fmt;

#[derive(Debug)]
pub enum RequestMethod {
    Get,
    Post,
}

impl fmt::Display for RequestMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RequestMethod::Get => write!(f, "GET"),
            RequestMethod::Post => write!(f, "POST"),
        }
    }
}
