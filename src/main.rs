mod evaluator;
mod lexer;
mod repl;
mod script;

use std::error::Error;

type RResult<T> = Result<T, Box<dyn Error>>;

fn main() {
    use std::{env, process};

    let args = env::args().skip(1).collect::<Vec<String>>();

    if args.is_empty() {
        repl::run();
    }

    for n in args {
        if let Err(e) = script::run(n) {
            eprintln!("{e}");
            process::exit(1);
        }
    }
}
