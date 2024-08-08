use anyhow::Result;
use diesel::SqliteConnection;
use guess_the_animal::database_utils::{create_question, get_all_questions, get_question_by_id, update_question_by_id};
use guess_the_animal::models::Question;
use guess_the_animal::endings::last_question;
use guess_the_animal::establish_connection;
use guess_the_animal::utils::ask_yes_or_no;

fn ask(question_before: Option<Question>, connection: &mut SqliteConnection) -> Result<()> {
    let question = match &question_before {
        None => {
            get_question_by_id(1)?.unwrap()
        }
        Some(q) => {
            match ask_yes_or_no(q.content.as_str())? {
                true => get_question_by_id(q.yes_id.unwrap())?.unwrap(),
                false => get_question_by_id(q.no_id.unwrap())?.unwrap()
            }
        }
    };

    if question.is_last == 1 {
        last_question(question, question_before.unwrap(), connection)?;
        return Ok(());
    }

    ask(Some(question), connection)
}

fn init_questions(connection: &mut SqliteConnection) -> Result<Question> {
    if !get_all_questions()?.is_empty() {
        return Ok(get_question_by_id(1)?.unwrap())
    }

    let mut default_first_question = Question::new("Can it fly?", None, None);
    let mut default_yes_animal = Question::new_last("eagle");
    let mut default_no_animal = Question::new_last("cow");

    default_first_question = create_question(connection, default_first_question)?;
    default_yes_animal = create_question(connection, default_yes_animal)?;
    default_no_animal = create_question(connection, default_no_animal)?;
    default_first_question = update_question_by_id(connection, default_first_question.id, None, Some(default_yes_animal.id), Some(default_no_animal.id))?;

    Ok(default_first_question)
}

fn main() -> Result<()> {
    let mut connection = establish_connection()?;

    println!("think of an animal\n");

    let first_question = init_questions(&mut connection)?;
    ask(Some(first_question), &mut connection)
}