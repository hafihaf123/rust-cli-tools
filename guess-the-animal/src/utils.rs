use std::io::{stdin, stdout, Write};

use anyhow::{Context, Result};

pub fn ask_yes_or_no(s: &str) -> Result<bool> {
    print!("{}\t(y/n):\t", s);
    stdout().flush()?;
    let mut answer: String = String::new();

    stdin().read_line(&mut answer).with_context(|| "Your input was not a valid UTF-8 character")?;

    return match answer.to_lowercase().trim() {
        "y" | "yes" => Ok(true),
        "n" | "no" => Ok(false),

        _ => ask_yes_or_no(s),
    }
}