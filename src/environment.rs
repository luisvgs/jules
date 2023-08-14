use crate::error::*;
use crate::value::*;
use anyhow::{anyhow, Result};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Default, Clone)]
pub struct Env {
    pub parent: Option<Rc<RefCell<Env>>>,
    pub vars: std::collections::HashMap<String, Expr>,
}

impl Env {
    pub fn new() -> Rc<RefCell<Self>> {
        let env = Rc::new(RefCell::new(Self::default()));
        env.borrow_mut().bind(
            "baz".into(),
            Expr::Primitive("baz".into(), |expr: Vec<Expr>| match &expr[..] {
                [Expr::Int(a), Expr::Int(b)] => Ok(Expr::Int(a + b)),
                x => Err(anyhow!(JError::EnvironmentError(format!(
                    "Expected a function, but got {:?}",
                    x
                )))),
            }),
        );
        env.borrow_mut().bind(
            "or".into(),
            Expr::Primitive("or".into(), |expr: Vec<Expr>| match &expr[..] {
                [Expr::Bool(a), Expr::Bool(b)] => Ok(Expr::Bool(*a || *b)),
                x => Err(anyhow!(JError::EnvironmentError(format!(
                    "expected (or Bool Bool) but got: {:?}",
                    x
                )))),
            }),
        );
        env.borrow_mut().bind(
            "and".into(),
            Expr::Primitive("and".into(), |expr: Vec<Expr>| match &expr[..] {
                [Expr::Bool(a), Expr::Bool(b)] => Ok(Expr::Bool(*a && *b)),
                x => Err(anyhow!(JError::EnvironmentError(format!(
                    "expected (and Bool Bool) but got: {:?}",
                    x
                )))),
            }),
        );
        env
    }

    pub fn extend(env: Rc<RefCell<Self>>) -> Self {
        Self {
            parent: Some(env),
            vars: std::collections::HashMap::new(),
        }
    }

    pub fn bind(&mut self, name: String, expr: Expr) {
        self.vars.insert(name, expr);
    }

    pub fn lookup(&mut self, name: String) -> Option<Expr> {
        self.vars.get(&name).cloned()
    }
}
