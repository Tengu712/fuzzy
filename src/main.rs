mod evaluator;
mod lexer;
mod repl;

use std::error::Error;

type RResult<T> = Result<T, Box<dyn Error>>;

fn main() {
    use std::env;

    let args = env::args().skip(1).collect::<Vec<String>>();

    if args.is_empty() {
        repl::run();
    }

    for n in args {
        println!("{n}");
    }
}
