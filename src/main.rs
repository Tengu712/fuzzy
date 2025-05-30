mod evaluator;
mod lexer;
mod repl;
mod script;

use std::error::Error;

type RResult<T> = Result<T, Box<dyn Error>>;

fn main() {
    use std::{env, process};

    let mut cmd_args = env::args().skip(1).collect::<Vec<String>>();

    if cmd_args.is_empty() {
        repl::run();
        return;
    }

    let args = cmd_args.split_off(1);
    let path = cmd_args.pop().unwrap();
    if let Err(e) = script::run(path, args) {
        eprintln!("{e}");
        process::exit(1);
    }
}
