use crate::ast::*;
use crate::environment::*;
use anyhow::Result;
use kinded::Kinded;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq, Kinded)]
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
            Self::List(l) => {
                write!(f, "(")?;
                let mut iter = l.iter().peekable();
                while let Some(expr) = iter.next() {
                    write!(f, "{}", expr)?;
                    if iter.peek().is_some() {
                        write!(f, " ")?;
                    }
                }
                write!(f, ")")
            }
            Self::Bool(b) => write!(f, "{}", b),
            Self::Function(_args, _body) => write!(f, "<fn:defined>"),
            Self::Primitive(name, _func) => write!(f, "<fn:{name}:primitive>"),
            Self::Nil => write!(f, "nil"),
        }
    }
}

impl std::ops::Add<i32> for Expr {
    type Output = Self;
    fn add(self, other: i32) -> Self {
        match self {
            Self::Int(x) => Expr::Int(x + other),
            _ => unreachable!(),
        }
    }
}

impl From<Expr> for i32 {
    fn from(expr: Expr) -> i32 {
        match expr {
            Expr::Int(n) => n,
            _ => unreachable!(),
        }
    }
}
