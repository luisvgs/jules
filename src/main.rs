mod ast;
mod environment;
mod error;
mod eval;
mod tests;
mod value;

use clap::{arg, command};
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

pub fn repl() {
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
                let mut foo: Vec<Expr> = nodes
                    .iter()
                    .map(|el| interpreter.eval_ast(el.clone()).unwrap())
                    .collect();

                println!("{:?}", foo);
            }
            Err(e) => panic!("Parsing error: {:?}", e),
        }
    }
}

pub fn from_file(input: &str) {
    let env = environment::Env::new();
    let mut interpreter = Interpreter::new(env);

    match parser::ExprsParser::new().parse(input) {
        Ok(nodes) => {
            let mut res = Expr::Nil;
            for e in nodes.iter() {
                res = interpreter.eval_ast(e.clone()).unwrap()
            }

            println!("{}", res);
        }
        Err(e) => panic!("Parsing error: {:?}", e),
    }
}

fn main() {
    let args = command!()
        .arg(arg!(-f <VALUE>).required(false))
        .get_matches();

    match args.get_one::<String>("file").map(|s| s.as_str()) {
        Some(file) => from_file(std::fs::read_to_string(file).unwrap().trim()),
        None => repl(),
    }
}
