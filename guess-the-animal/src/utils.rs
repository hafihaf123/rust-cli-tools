use std::io;
use anyhow::{Result, Context};

pub fn ask_yes_or_no(s: String) -> Result<bool> {
    print!("{}\t(y/n):\t", s);
    let mut answer: String = String::new();

    io::stdin().read_line(&mut answer).with_context(|| "Your input was not a valid UTF-8 character")?;

    match answer.to_lowercase().as_str() {
        "y" => Ok(true),
        "n" => Ok(false),

        _ => ask_yes_or_no(s),
    }
}