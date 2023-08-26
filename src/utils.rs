use crate::environment::Env;
use crate::eval::*;
use crate::value::Expr;
use lalrpop_util::lalrpop_mod;
use std::io::Write;
lalrpop_mod!(
    #[allow(clippy::all)]
    #[allow(clippy::pedantic)]
    #[allow(dead_code)]
    parser
);

pub fn repl() {
    let env = Env::new();
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
                let exprs: Vec<Expr> = nodes
                    .iter()
                    .map(|el| interpreter.eval_ast(el.clone()).unwrap())
                    .collect();

                println!("{:?}", exprs);
            }
            Err(e) => panic!("Parsing error: {:?}", e),
        }
    }
}

pub fn from_file(input: &str) {
    let env = Env::new();
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
