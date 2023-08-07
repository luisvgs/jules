use crate::ast::*;
use crate::value::*;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Default, Clone)]
pub struct Env {
    pub parent: Option<Rc<RefCell<Env>>>,
    pub vars: std::collections::HashMap<String, Value>,
}

impl Env {
    pub fn new() -> Rc<RefCell<Self>> {
        let env = Rc::new(RefCell::new(Self::default()));
        env.borrow_mut().bind(
            "baz".into(),
            Value::Primitive("baz".into(), |expr: Vec<Ast>| match &expr[..] {
                [Ast::Int(a), Ast::Int(b)] => Value::Int(a + b),
                x => unreachable!("unreachable expression caught: {:?}", x),
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

    pub fn bind(&mut self, name: String, expr: Value) {
        self.vars.insert(name, expr);
    }

    pub fn lookup(&mut self, name: String) -> Option<Value> {
        self.vars.get(&name).cloned()
    }
}
