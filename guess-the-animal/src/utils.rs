use std::io::{stdin, stdout, Write};

use anyhow::Result;

pub fn ask_yes_or_no(s: &str) -> Result<bool> {
    print!("{}\t(y/n):\t", s);
    stdout().flush()?;
    let mut answer: String = String::new();

    stdin().read_line(&mut answer)?;

    return match answer.to_lowercase().trim() {
        "y" | "yes" => Ok(true),
        "n" | "no" => Ok(false),

        _ => ask_yes_or_no(s),
    }
}