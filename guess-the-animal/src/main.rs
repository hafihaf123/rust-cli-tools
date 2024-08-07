use anyhow::Result;
use diesel::SqliteConnection;
use guess_the_animal::database_utils::{create_question, get_all_questions, get_question_by_id, update_question_by_id};
use guess_the_animal::diesel_files::Question;
use guess_the_animal::establish_connection;
use guess_the_animal::utils::ask_yes_or_no;

fn ask(i: i32) -> Result<()> {
    let question = get_question_by_id(i)?.unwrap();

    if question.is_last == 1 {
        // end by asking
        println!("{:?}", question);
        return Ok(());
    }

    match ask_yes_or_no(question.content.as_str())? {
        true => ask(question.yes_id.unwrap())?,
        false => ask(question.no_id.unwrap())?
    }

    Ok(())
}

fn init_questions(connection: &mut SqliteConnection) -> Result<()> {
    if !get_all_questions()?.is_empty() {
        return Ok(())
    }

    let mut default_first_question = Question::new("can it swim?", None, None);
    let mut default_yes_question = Question::new_last("Is it a fish?", None, None);
    let mut default_no_question = Question::new_last("Is it an eagle?", None, None);

    default_first_question.id = create_question(connection, &default_first_question)?;
    default_yes_question.id = create_question(connection, &default_yes_question)?;
    default_no_question.id = create_question(connection, &default_no_question)?;
    update_question_by_id(connection, default_first_question.id, None, Some(default_yes_question.id), Some(default_no_question.id))?;

    Ok(())
}

fn main() -> Result<()> {
    let mut connection = establish_connection()?;

    println!("think of an animal\n");

    init_questions(&mut connection)?;
    ask(1)
}