use crate::ast::*;
use crate::environment::*;
use anyhow::Result;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Int(i32),
    Bool(bool),
    Symbol(String),
    List(Vec<Expr>),
    Function(Vec<String>, Box<Ast>),
    Primitive(String, fn(Vec<Expr>, Rc<RefCell<Env>>) -> Result<Expr>),
    Nil,
}

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Int(a) => write!(f, "{}", a),
            Self::Symbol(s) => write!(f, "{}", s),
            Self::List(l) => write!(f, "{:?}", l),
            Self::Bool(b) => write!(f, "{}", b),
            Self::Function(args, body) => write!(f, "<fn: defined>"),
            Self::Primitive(name, func) => write!(f, "<fn:{name} is primitive>: {:?}", func),
            Self::Nil => write!(f, "nil"),
        }
    }
}
