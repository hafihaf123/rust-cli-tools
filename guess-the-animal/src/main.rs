use anyhow::Result;
use guess_the_animal::establish_connection;
use guess_the_animal::database_utils as database;

fn main() -> Result<()> {
    println!("think of an animal\n");

    database::get_questions()?;
    println!();
    database::get_animals()?;
    println!();

    let mut conn = establish_connection()?;

    println!("adding a new question...");
    database::create_question(&mut conn, "can it roar?")?;
    database::get_questions()?;

    println!();

    println!("added a new animal!!");
    database::create_animal(&mut conn, "horse")?;
    database::get_animals()?;

    Ok(())
}