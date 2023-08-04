#[derive(Debug, Clone)]
pub enum Value {
    Int(i32),
    Nil,
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Int(a) => write!(f, "{}", a),
            Self::Nil => write!(f, "nil"),
        }
    }
}
