mod ast;
mod environment;
mod error;
mod eval;
mod tests;
mod utils;
mod value;
use clap::{arg, command};
use utils::*;
extern crate lalrpop_util;

// TODO
// keywords to implement:
//
// [] atom?
// [] eq
// [] car
// [] cdr
// [] cons/pair
// [] quote
// [] cond/if
// [] lambda
// [] label/define

fn main() {
    let args = command!()
        .arg(arg!(--file <VALUE>).required(false))
        .get_matches();

    match args.get_one::<String>("file").map(|s| s.as_str()) {
        Some(file) => from_file(std::fs::read_to_string(file).unwrap().trim()),
        None => repl(),
    }
}
