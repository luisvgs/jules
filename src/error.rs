#[derive(Debug, Clone)]
pub enum JError {
    InvalidOperation,
    ParsingError(String),
}

impl std::fmt::Display for JError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidOperation => write!(f, "Doesn't look too bad"),
            Self::ParsingError(cause) => {
                write!(
                    f,
                    "An error ocurred while parsing the AST. Cause: {}",
                    cause
                )
            }
        }
    }
}
