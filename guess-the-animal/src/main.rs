use std::io::BufRead;
use anyhow::{Context, Result};

mod endings;
mod utils;
mod lib;

use lib::establish_connection;

fn ask(line: Line) -> Result<String> {
    let answer = utils::ask_yes_or_no(line.question)?;
    match answer {
        true => {
            match file_utils::get_line_from_file(line.n + 1)? {
                None => endings::animal_guess(line.animal)?;
                Some(next_line) => ask(next_line)
            }
        },
        false => Ok(line.animal)
    }
}

fn game_start() -> Result<()> {
    match file_utils::get_line_from_file(0)? {
        None => {},
        Some(question) => {
            ask(question)?;
        },
    }
    Ok(())
}

fn main() -> Result<()> {
    println!("think of an animal\n");
    
    game_start()?;

    Ok(())
}
