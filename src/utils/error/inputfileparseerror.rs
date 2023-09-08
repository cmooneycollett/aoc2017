use std::fmt;

/// Custom error type indicating that the parsing of the input file has failed.
#[derive(Debug)]
pub struct InputFileParseError {
    pub message: String,
}

impl fmt::Display for InputFileParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Parsing of input file failed: {}", self.message)
    }
}
