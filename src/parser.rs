use crate::EError;
use std::io::{BufRead, BufReader, Read};

enum Mode {
    Blank,
}

pub fn parse<R: Read>(content: R) -> Result<(), EError> {
    let mode = Mode::Blank;
    for l in BufReader::new(content).lines() {
        for c in l?.chars() {
            match mode {
                Mode::Blank => match c {
                    ' ' | '\t' | '\r' | '\n' => (),
                    _ => (), // TODO: push token buffer.
                },
            }
        }
    }
    Ok(())
}
