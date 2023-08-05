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

//NOTE: Support for booleans (#t and #f) and more binary operations.
fn main() {
    let env = environment::Env::new();
    let ast = parser::ExprsParser::new()
        .parse(
            "
            (defun add (a:Int, b:Int) :Int (3))
            ",
        )
        .unwrap();
    let foo: Vec<Value> = ast
        .iter()
        .map(|el| eval_ast(el.clone(), env.clone()).unwrap())
        .collect();

    println!("{:?}", foo);
}
