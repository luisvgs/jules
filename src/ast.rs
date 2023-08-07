#[derive(Debug, Clone)]
pub enum Ast {
    Int(i32),
    Bool(bool),
    Symbol(String),
    List(Vec<Ast>),
    Var(String, Box<Ast>),
    Function(String, Vec<String>, Box<Ast>),
    Nil,
}
impl std::fmt::Display for Ast {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Int(a) => write!(f, "{}", a),
            Self::Function(name, args, body) => write!(f, "<fn: defined>"),
            Self::Nil => write!(f, "nil"),
            Self::Symbol(sym) => write!(f, "{}", sym),
            _ => unreachable!(),
        }
    }
}
