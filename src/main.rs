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
        let input = input.trim_end();

        let tokens = lexer::lex(input)?;

        match tokens.get(0) {
            Some(lexer::Token::Symbol(n)) if n == "#exit" => break,
            _ => (),
        }

        println!("{:?}", tokens);
    }

    Ok(())
}

fn main() {
    // TODO: print
    run().unwrap();
}
