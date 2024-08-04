use std::io;
use anyhow::{Result, Context};

pub fn ask_yes_or_no(s: String) -> Result<bool> {
    print!("{}\t(y/n):\t", s);
    let mut answer: String;

    io::stdin().read_line(&mut answer).with_context(|| "Your input was not a valid UTF-8 character")?;

    match answer.to_lowercase() {
        String::from("y") => Ok(true),
        String::from("n") => Ok(false),

        _ => ask_yes_or_no(s),
    }
}