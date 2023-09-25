use crate::error::*;
use crate::value::*;
use anyhow::anyhow;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Env {
    pub parent: Option<Rc<RefCell<Env>>>,
    pub vars: std::collections::HashMap<String, Expr>,
}

impl Env {
    pub fn new() -> Rc<RefCell<Self>> {
        let env = Rc::new(RefCell::new(Self::default()));
        env.borrow_mut().bind(
            ">".into(),
            Expr::Primitive(
                "<".into(),
                |expr: Vec<Expr>, _env: Rc<RefCell<Env>>| match &expr[..] {
                    [Expr::Int(a), Expr::Int(b)] => Ok(Expr::Bool(*a > *b)),
                    x => Err(anyhow!(JError::EnvironmentError(format!(
                        "expected (> Bool Bool) but got: {:?}",
                        x
                    )))),
                },
            ),
        );
        env.borrow_mut().bind(
            "=".into(),
            Expr::Primitive(
                "=".into(),
                |expr: Vec<Expr>, _env: Rc<RefCell<Env>>| match &expr[..] {
                    [Expr::Int(a), Expr::Int(b)] => Ok(Expr::Bool(*a == *b)),
                    x => Err(anyhow!(JError::EnvironmentError(format!(
                        "expected (= Bool Bool) but got: {:?}",
                        x
                    )))),
                },
            ),
        );
        env.borrow_mut().bind(
            "<".into(),
            Expr::Primitive(
                "<".into(),
                |expr: Vec<Expr>, _env: Rc<RefCell<Env>>| match &expr[..] {
                    [Expr::Int(a), Expr::Int(b)] => Ok(Expr::Bool(*a < *b)),
                    x => Err(anyhow!(JError::EnvironmentError(format!(
                        "expected (< Bool Bool) but got: {:?}",
                        x
                    )))),
                },
            ),
        );
        env.borrow_mut().bind(
            "eq".into(),
            Expr::Primitive(
                "eq".into(),
                |expr: Vec<Expr>, _env: Rc<RefCell<Env>>| match &expr[..] {
                    [Expr::Bool(a), Expr::Bool(b)] => Ok(Expr::Bool(*a == *b)),
                    [Expr::Int(a), Expr::Int(b)] => Ok(Expr::Bool(*a == *b)),
                    x => Err(anyhow!(JError::EnvironmentError(format!(
                        "expected (eq atom atom) but got: {:?}",
                        x
                    )))),
                },
            ),
        );
        env.borrow_mut().bind(
            "or".into(),
            Expr::Primitive(
                "or".into(),
                |expr: Vec<Expr>, _env: Rc<RefCell<Env>>| match &expr[..] {
                    [Expr::Bool(a), Expr::Bool(b)] => Ok(Expr::Bool(*a || *b)),
                    x => Err(anyhow!(JError::EnvironmentError(format!(
                        "expected (or Bool Bool) but got: {:?}",
                        x
                    )))),
                },
            ),
        );
        env.borrow_mut().bind(
            "print".into(),
            Expr::Primitive(
                "print".into(),
                |expr: Vec<Expr>, env: Rc<RefCell<Env>>| match &expr[..] {
                    [Expr::Symbol(s)] => {
                        let get_val = env.borrow_mut().lookup(s.to_string()).unwrap();
                        Ok(get_val)
                    }
                    [e] => Ok(e.clone()),
                    x => Err(anyhow!(JError::EnvironmentError(format!(
                        "print expected arguments, but got: {:?}",
                        x
                    )))),
                },
            ),
        );
        env.borrow_mut().bind(
            "cdr".into(),
            Expr::Primitive(
                "cdr".into(),
                |expr: Vec<Expr>, _env: Rc<RefCell<Env>>| match &expr[..] {
                    [Expr::List(head), ..] => Ok(head.iter().nth(1).unwrap().clone()),
                    x => Err(anyhow!(JError::EnvironmentError(format!(
                        "cdr expected arguments, but got: {:?}",
                        x
                    )))),
                },
            ),
        );
        env.borrow_mut().bind(
            "car".into(),
            Expr::Primitive(
                "car".into(),
                |expr: Vec<Expr>, _env: Rc<RefCell<Env>>| match &expr[..] {
                    [Expr::List(head), ..] => Ok(head.first().unwrap().clone()),
                    x => Err(anyhow!(JError::EnvironmentError(format!(
                        "car expected arguments, but got: {:?}",
                        x
                    )))),
                },
            ),
        );
        env.borrow_mut().bind(
            "list".into(),
            Expr::Primitive(
                "list".into(),
                |expr: Vec<Expr>, _env: Rc<RefCell<Env>>| match &expr[..] {
                    [head, tail @ ..] => {
                        let mut els = Vec::new();
                        els.push(head.clone());
                        for el in tail.iter() {
                            els.push(el.clone())
                        }
                        Ok(Expr::List(els))
                    }
                    x => Err(anyhow!(JError::EnvironmentError(format!(
                        "list expected arguments, but got: {:?}",
                        x
                    )))),
                },
            ),
        );
        env.borrow_mut().bind(
            "and".into(),
            Expr::Primitive(
                "and".into(),
                |expr: Vec<Expr>, _env: Rc<RefCell<Env>>| match &expr[..] {
                    [Expr::Bool(a), Expr::Bool(b)] => Ok(Expr::Bool(*a && *b)),
                    x => Err(anyhow!(JError::EnvironmentError(format!(
                        "expected (and Bool Bool) but got: {:?}",
                        x
                    )))),
                },
            ),
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
