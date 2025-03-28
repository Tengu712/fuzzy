use std::error::Error;

type RResult<T> = Result<T, Box<dyn Error>>;

fn run() -> RResult<()> {
    use std::io::{self, Write};

    loop {
        print!(">> ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        // TODO: *exit* @
        if input.trim() == "*exit*" {
            break;
        }

        println!("{input}");
    }

    Ok(())
}

fn main() {
    // TODO: print
    run().unwrap();
}
