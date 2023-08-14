use crate::ast::*;
use crate::environment::*;
use crate::error::*;
use crate::value::*;
use anyhow::{anyhow, Result};
use std::cell::RefCell;
use std::rc::Rc;

pub struct Interpreter {
    pub env: Rc<RefCell<Env>>,
}

impl Interpreter {
    pub fn new(env: Rc<RefCell<Env>>) -> Self {
        Self { env }
    }
    pub fn eval_block(&mut self, stmts: Box<Ast>, env: Rc<RefCell<Env>>) -> Result<Expr> {
        let mut value: Expr = Expr::Nil;
        let previous = self.env.clone();
        let steps = || -> Result<Expr> {
            self.env = env;
            value = self.eval_ast(*stmts).unwrap();
            Ok(value)
        };
        let result = steps();
        self.env = previous;

        result
    }
    pub fn eval_ast(&mut self, ast: Ast) -> Result<Expr> {
        match ast {
            Ast::Int(a) => Ok(Expr::Int(a)),
            Ast::Bool(b) => Ok(Expr::Bool(b)),
            Ast::List(list) => match &list[..] {
                [Ast::Int(a)] => Ok(Expr::Int(*a)),
                [Ast::Bool(b)] => match b {
                    true => Ok(Expr::Bool(true)),
                    _ => Ok(Expr::Bool(false)),
                },
                [Ast::Symbol(sym), _x @ ..] if sym == "+" => {
                    let foo = list.clone().iter().fold(0, |acc, num| {
                        if let Ast::Int(val) = num {
                            val + acc
                        } else {
                            acc
                        }
                    });

                    Ok(Expr::Int(foo))
                }
                [Ast::Symbol(s)] => {
                    let get_val = self.env.borrow_mut().lookup(s.to_string()).unwrap();
                    Ok(get_val)
                }
                [Ast::Symbol(f_name), args @ ..] => {
                    let eval_f = self.env.borrow_mut().lookup(f_name.to_string()).unwrap();
                    let args_: Vec<Expr> = args
                        .into_iter()
                        .map(|e| self.eval_ast(e.clone()).unwrap())
                        .collect();
                    match eval_f {
                        Expr::Primitive(_, f) => Ok(f(args_)?),
                        Expr::Function(params, body) => {
                            let mut vals = Vec::new();

                            for arg in args {
                                match self.eval_ast(arg.clone()) {
                                    Ok(v) => vals.push(v),
                                    Err(e) => {
                                        return Err(anyhow!(JError::ParsingError(e.to_string())))
                                    }
                                }
                            }
                            let environment = Rc::new(RefCell::new(Env::extend(self.env.clone())));
                            for (param, argument) in params.iter().zip(vals.iter()) {
                                environment
                                    .borrow_mut()
                                    .bind(param.clone(), argument.clone());
                            }
                            self.eval_block(body, environment)
                        }
                        _ => Err(anyhow!(JError::InvalidOperation(format!(
                            "Expected a function, but got {:?}",
                            eval_f.clone()
                        )))),
                    }
                }
                [Ast::Var(name, value)] => {
                    let ev_val = self.eval_ast(*value.clone()).unwrap();
                    self.env.borrow_mut().bind(name.into(), ev_val);
                    Ok(Expr::Nil)
                }
                [Ast::Function(name, args, body)] => {
                    let f = Expr::Function(args.to_vec(), body.clone());
                    self.env.borrow_mut().bind(name.into(), f.clone());
                    Ok(f.clone())
                }
                e => Err(anyhow!(JError::UnimplementedFeature(format!(
                    "{:?}",
                    e.clone()
                )))),
            },
            error => Err(anyhow!(JError::UnimplementedFeature(error.to_string()))),
        }
    }
}
