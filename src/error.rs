use std::error;
use std::fmt;
use std::io;

/// Raised when incorrect option is given during normal runtime
#[derive(Debug)]
pub struct OptionError {
    details: String,
}

impl OptionError {
    pub fn new(msg: String) -> OptionError {
        OptionError { details: msg }
    }
}

impl fmt::Display for OptionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl error::Error for OptionError {
    fn description(&self) -> &str {
        &self.details
    }
}

impl From<OptionError> for io::Error {
    fn from(err: OptionError) -> Self {
        io::Error::new(io::ErrorKind::Other, err)
    }
}
