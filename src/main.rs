mod evaluator;
mod lexer;
mod repl;

use std::error::Error;

type RResult<T> = Result<T, Box<dyn Error>>;

fn main() {
    repl::run();
}
