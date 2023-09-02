use crate::ast::*;
use crate::environment::Env;
use crate::eval::*;
use crate::value::Expr;

use lalrpop_util::lalrpop_mod;
lalrpop_mod!(
    #[allow(clippy::all)]
    #[allow(clippy::pedantic)]
    #[allow(dead_code)]
    parser
);

macro_rules! int {
    ($i:expr) => {
        Expr::Int($i)
    };
}

macro_rules! bool {
    ($b: expr) => {
        Expr::Bool($b)
    };
}

macro_rules! list {
    ($b: expr) => {
        Expr::List($b)
    };
}

#[test]
fn value_assignment_and_call_works() {
    let env = Env::new();
    let mut interpreter = Interpreter::new(env);
    let parsed_expr = parser::ExprsParser::new()
        .parse("(val x :Int 10)\n(x)")
        .unwrap();

    let mut exprs = Expr::Nil;
    for e in parsed_expr.iter() {
        exprs = interpreter.eval_ast(e.clone()).unwrap()
    }
    assert_eq!(exprs, int!(10))
}
#[test]
fn parse_values_works() {
    let env = Env::new();
    let mut interpreter = Interpreter::new(env);
    let cases = vec![
        ("(282)", int!(282)),
        ("(#t)", bool!(true)),
        ("(#f)", bool!(false)),
    ];

    let mut exprs = Expr::Nil;
    for (input, expected) in cases.iter() {
        let parsed_expr = parser::ExprsParser::new().parse(input).unwrap();
        for e in parsed_expr.iter() {
            exprs = interpreter.eval_ast(e.clone()).unwrap()
        }
        assert_eq!(exprs, expected.clone());
    }
}

#[test]
fn lt_gt_operator_works() {
    let env = Env::new();
    let mut interpreter = Interpreter::new(env);
    let cases = vec![
        ("(> 100 50)", bool!(true)),
        ("(> 50 100)", bool!(false)),
        ("(< 20 50)", bool!(true)),
        ("(< 88 12)", bool!(false)),
    ];

    let mut exprs = Expr::Nil;
    for (input, expected) in cases.iter() {
        let parsed_expr = parser::ExprsParser::new().parse(input).unwrap();
        for e in parsed_expr.iter() {
            exprs = interpreter.eval_ast(e.clone()).unwrap()
        }
        assert_eq!(exprs, expected.clone());
    }
}
#[test]
fn cat_cdr_works() {
    let env = Env::new();
    let mut interpreter = Interpreter::new(env);
    let cases = vec![
        ("(list 1 2 3)", list!(vec![int!(1), int!(2), int!(3)])),
        ("(cdr (list 1 2 3))", int!(2)),
        ("(car (list 1 2 3))", int!(1)),
    ];

    let mut exprs = Expr::Nil;
    for (input, expected) in cases.iter() {
        let parsed_expr = parser::ExprsParser::new().parse(input).unwrap();
        for e in parsed_expr.iter() {
            exprs = interpreter.eval_ast(e.clone()).unwrap()
        }
        assert_eq!(exprs, expected.clone());
    }
}
