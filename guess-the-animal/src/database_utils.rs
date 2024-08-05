use crate::models::*;
use diesel::prelude::*;
use anyhow::{Context, Result};
// use crate::schema::questions::dsl::questions;

pub fn get_questions() -> Result<()> {
    use crate::schema::questions::dsl::*;

    let connection = &mut crate::establish_connection()?;
    let results = questions
        .limit(5)
        .select(Question::as_select())
        .load(connection)
        .with_context(|| "Error loading questions")?;

    println!("Displaying {} questions", results.len());
    for q in results {
        println!("({}): {}", q.id, q.question);
    }
    Ok(())
}

pub fn get_animals() -> Result<()> {
    use crate::schema::animals::dsl::*;

    let connection = &mut crate::establish_connection()?;
    let results = animals
        .limit(5)
        .select(Animal::as_select())
        .load(connection)
        .with_context(|| "Error loading animals")?;

    println!("Displaying {} animals", results.len());
    for animal in results {
        println!("({}): {}", animal.id, animal.name);
    }
    Ok(())
}

pub fn create_question(connection: &mut SqliteConnection, question: &str) -> Result<Question> {
    use crate::schema::questions;

    let new_question = NewQuestion { question };

    Ok(diesel::insert_into(questions::table)
        .values(&new_question)
        .returning(Question::as_returning())
        .get_result(connection)
        .with_context(|| "Could not save the question")?)
}