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

fn main() {
    let env = environment::Env::new();
    let ast = parser::ExprsParser::new()
        .parse(
            "
            (val foo :Int 20)
            (foo)
            ",
        )
        .unwrap();
    let foo: Vec<Value> = ast
        .iter()
        .map(|el| eval_ast(el.clone(), env.clone()).unwrap())
        .collect();

    println!("{:?}", foo);
}
