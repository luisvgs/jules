#[derive(Debug)]
pub enum JError {
    InvalidOperation(String),
    ParsingError(String),
    UnimplementedFeature(String),
    EnvironmentError(String),
}

impl std::fmt::Display for JError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidOperation(cause) => {
                write!(f, "error: this operation is not valid. {}", cause)
            }
            Self::UnimplementedFeature(cause) => {
                write!(f, "This feature is yet to be implemented: {}", cause)
            }
            Self::ParsingError(cause) => {
                write!(
                    f,
                    "An error ocurred while parsing the AST. Reason: {}",
                    cause
                )
            }
            Self::EnvironmentError(cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
