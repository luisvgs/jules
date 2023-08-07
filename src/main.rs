mod ast;
mod environment;
mod eval;
mod value;
use eval::*;
use std::io::Write;
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

//TODO:
//Error handling
// Nested variable scoping
pub fn run() {
    let env = environment::Env::new();
    let mut interpreter = Interpreter::new(env);
    loop {
        print!(":> ");
        std::io::stdout().flush().unwrap();
        let mut line = String::new();
        std::io::stdin()
            .read_line(&mut line)
            .expect("Unable to read line from the REPL");
        if line.is_empty() || line.contains(":q") {
            break;
        }
        match parser::ExprsParser::new().parse(&line) {
            Ok(nodes) => {
                let foo: Vec<Expr> = nodes
                    .iter()
                    .map(|el| interpreter.eval_ast(el.clone()).unwrap())
                    .collect();

                println!("{:?}", foo);
            }
            Err(e) => println!("Whupps"),
        }
    }
}
fn main() {
    run();
}
