mod evaluator;
mod lexer;

use std::error::Error;

type RResult<T> = Result<T, Box<dyn Error>>;

fn run() -> RResult<()> {
    use std::io::{self, Write};

    loop {
        print!(">> ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();

        if input == "#exit" {
            break;
        }

        let mut tokens = lexer::lex(input)?;
        tokens.reverse();
        let value = evaluator::eval(tokens)?;
        println!("{value:?}");
    }

    Ok(())
}

fn main() {
    // TODO: print
    run().unwrap();
}
