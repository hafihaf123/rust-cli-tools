use anyhow::Result;

use guess_the_animal::database_utils::{create_question, get_animals, get_questions};
use guess_the_animal::establish_connection;

fn main() -> Result<()> {
    println!("think of an animal\n");

    get_questions()?;
    println!();
    get_animals()?;
    println!();

    let mut conn = establish_connection()?;
    create_question(&mut conn, "can it roar?")?;

    println!("added a new question!!");
    get_questions()?;

    Ok(())
}