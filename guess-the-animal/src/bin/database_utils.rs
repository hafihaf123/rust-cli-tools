use guess_the_animal::models::*;
use diesel::prelude::*;
use anyhow::Result;

fn main() -> Result<()> {
    use guess_the_animal::schema::questions::dsl::*;

    let connection = &mut guess_the_animal::establish_connection()?;
    let results = questions
        .limit(5)
        .select(Question::as_select())
        .load(connection)
        .expect("Error loading posts");

    println!("Displaying {} questions", results.len());
    for q in results {
        println!("{}", q.question);
    }
    Ok(())
}