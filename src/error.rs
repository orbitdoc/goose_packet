use std;
use std::fmt;

#[derive(Debug, Clone)]
pub struct GooseError {
    pub message: String,
    pub pos: usize,
}

// Errors should be printable.
impl fmt::Display for GooseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{} ({})", self.message, self.pos)
    }
}

// Errors should implement the std::error::Error trait

impl std::error::Error for GooseError {
    fn description(&self) -> &str {
        &self.message
    }
}
