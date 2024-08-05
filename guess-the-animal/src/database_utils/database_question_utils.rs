use diesel::SqliteConnection;
use crate::diesel_files::models::Question;

pub fn get_questions() -> anyhow::Result<()> {
    use diesel::{QueryDsl, RunQueryDsl, SelectableHelper};
    use anyhow::Context;
    use crate::diesel_files::schema::questions::dsl::*;

    let connection = &mut crate::establish_connection()?;
    let results = questions
        .limit(5)
        .select(Question::as_select())
        .load(connection)
        .with_context(|| "Could not load questions")?;

    println!("Displaying {} questions", results.len());
    for question in results {
        println!("({}): {}", question.id, question.content);
    }
    Ok(())
}

pub fn get_question_by_id(id: i32) -> anyhow::Result<()> {
    use diesel::{OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper};
    use anyhow::Context;
    use crate::diesel_files::models::Question;
    use crate::diesel_files::schema::questions::dsl::questions;

    let connection = &mut crate::establish_connection()?;
    let question = questions
        .find(id)
        .select(Question::as_select())
        .first(connection)
        .optional()
        .with_context(|| format!("Could not load question with id `{}`", id))?;

    match question {
        Some(question) => {
            println!("({}): {}", question.id, question.content);
        },
        None => {
            println!("There is no question with id `{}`", id);
        },
    }
    Ok(())
}

pub fn create_question(connection: &mut SqliteConnection, question: &str) -> anyhow::Result<Question> {
    use diesel::{RunQueryDsl, SelectableHelper};
    use anyhow::Context;
    use crate::diesel_files::models::NewQuestion;
    use crate::diesel_files::schema::questions;

    let new_question = NewQuestion { question };

    Ok(diesel::insert_into(questions::table)
        .values(&new_question)
        .returning(Question::as_returning())
        .get_result(connection)
        .with_context(|| "Could not save the question")?)
}

pub fn delete_question_by_id(connection: &mut SqliteConnection, n: i32) -> anyhow::Result<usize> {
    use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
    use anyhow::Context;
    use crate::diesel_files::schema::questions::dsl::*;

    Ok(diesel::delete(questions.filter(id.eq(n)))
        .execute(connection)
        .with_context(|| format!("Could not delete question with id `{}`", n))?)
}

pub fn delete_question_by_content(connection: &mut SqliteConnection, content_: String) -> anyhow::Result<usize> {
    use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
    use anyhow::Context;
    use crate::diesel_files::schema::questions::dsl::*;

    Ok(diesel::delete(questions.filter(content.eq(content_)))
        .execute(connection)
        .with_context(|| format!("Could not delete question with content `{}`", content_))?)
}