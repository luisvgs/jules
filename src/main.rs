mod ast;
mod environment;
mod eval;
mod value;

use eval::*;
use value::*;

#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(
    #[allow(clippy::all)]
    #[allow(clippy::pedantic)]
    #[allow(dead_code)]
    parser
);

//NOTE:
// ;; Function declaration with type annotations
// (defun add (a:Int, b:Int) : Int
//   (+ a b))
// ;; Variable declaration with type annotation
// (val x :Int 10)
// ;; Function call with type annotations
// (let ((result : number (add x 5)))
//   (print result))

fn main() {
    let env = environment::Env::new();
    let mut interpreter = Interpreter::new(env.clone());
    let ast = parser::ExprsParser::new()
        .parse(
            "
            (defun add (a:Int, b:Int) :Int
                (+ 9 1))
            (add 3 4)

            (or #f #f)
            ",
        )
        .unwrap();
    let foo: Vec<Value> = ast
        .iter()
        .map(|el| interpreter.eval_ast(el.clone()).unwrap())
        .collect();

    println!("res: {:?}", foo);
}
