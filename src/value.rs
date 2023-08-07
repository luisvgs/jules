use crate::ast::*;

#[derive(Debug, Clone)]
pub enum Expr {
    Int(i32),
    Bool(bool),
    Function(Vec<String>, Box<Ast>),
    Primitive(String, fn(Vec<Ast>) -> Expr),
    Nil,
}

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Int(a) => write!(f, "{}", a),
            Self::Bool(b) => write!(f, "{}", b),
            Self::Function(args, body) => write!(f, "<fn: defined>"),
            Self::Primitive(name, func) => write!(f, "<fn:{name} is primitive>: {:?}", func),
            Self::Nil => write!(f, "nil"),
        }
    }
}
