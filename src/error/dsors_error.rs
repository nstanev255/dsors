use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct DsorsError {
    message: String
}

impl DsorsError {
    pub fn new(message: &str) -> DsorsError {
        DsorsError { message: message.to_string() }
    }
}

impl Display for DsorsError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for DsorsError {
    fn description(&self) -> &str {
        &self.message
    }
}