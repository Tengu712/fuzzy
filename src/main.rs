mod lexer;
mod parser;

type EError = Box<dyn std::error::Error>;

fn run() -> Result<(), EError> {
    use std::{env, fs::File};

    let args = env::args().skip(1).collect::<Vec<String>>();

    if args.len() > 2 {
        return Err("too much arguments passed.".to_string().into());
    }
    if args.is_empty() {
        // TODO: implement REPL.
        return Ok(());
    }

    // TODO: get tokens and pass them to evaluator.
    let tokens = lexer::lex(File::open(&args[0])?)?;
    let _ = parser::parse(tokens)?;

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("error: {}", e);
        std::process::exit(1);
    }
}
