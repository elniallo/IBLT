use std::error::Error;
use std::fmt;
#[derive(Debug)]
pub struct IBLTError {
    details: String,
}

impl IBLTError {
    pub fn new(msg: &str) -> IBLTError {
        IBLTError {
            details: String::from(msg),
        }
    }
}

impl fmt::Display for IBLTError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for IBLTError {
    fn description(&self) -> &str {
        &self.details
    }
}
