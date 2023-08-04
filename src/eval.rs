use crate::ast::*;
use crate::environment::*;
use crate::value::*;
use std::cell::RefCell;
use std::rc::Rc;

pub fn eval_ast(ast: Ast, env: Rc<RefCell<Env>>) -> Result<Value, String> {
    match ast {
        Ast::List(list) => match &list[..] {
            [Ast::Symbol(sym), _x @ ..] if sym == "+" => {
                let foo = list.clone().iter().fold(0, |acc, num| {
                    if let Ast::Int(val) = num {
                        val + acc
                    } else {
                        acc
                    }
                });

                Ok(Value::Int(foo))
            }
            [Ast::Var(name, value)] => {
                let ev_val = eval_ast(*value.clone(), env.clone()).unwrap();
                env.borrow_mut().bind(name.into(), ev_val);
                Ok(Value::Nil)
            }
            [Ast::Int(a)] => Ok(Value::Int(*a)),
            [Ast::Symbol(s)] => {
                let get_val = env.borrow_mut().lookup(s.to_string()).unwrap();

                Ok(get_val)
            }
            x => unimplemented!("Unimplemented expression: {:?}", x),
        },
        Ast::Int(a) => Ok(Value::Int(a)),
        error => unimplemented!("Unimplemented feature caught: {:?}", error),
    }
}
