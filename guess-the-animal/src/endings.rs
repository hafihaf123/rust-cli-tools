use std::io::stdin;
use crate::models::Question;
use crate::utils::ask_yes_or_no;
use anyhow::{anyhow, Result};
use diesel::SqliteConnection;
use crate::database_utils::{create_question, update_question_by_id};

pub fn last_question(old_animal: Question, question_before: Question, connection: &mut SqliteConnection) -> Result<()> {
    if old_animal.is_last != 1 {
        return Err(anyhow!("The question must have `is_last` set to `1` (true).\nThe question: {:?}", old_animal));
    }

    if ask_yes_or_no(old_animal.content.as_str())? {
        println!("Yay!!");
        return Ok(());
    }

    println!("What was the animal you thought of?");
    let mut animal_name = String::new();
    stdin().read_line(&mut animal_name)?;

    println!("What would you ask to differ it? (the answer would be `yes` for your animal)?");
    let mut new_question_content = String::new();
    stdin().read_line(&mut new_question_content)?;

    let mut new_animal = Question::new_last(animal_name.trim());
    new_animal = create_question(connection, new_animal)?;

    let mut new_question = Question::new(
        new_question_content.trim(),
        Some(new_animal.id),
        Some(old_animal.id)
    );
    new_question = create_question(connection, new_question)?;

    let was_yes = if old_animal.id == question_before.yes_id.unwrap() {
        true
    } else if old_animal.id == question_before.no_id.unwrap() {
        false
    } else {
        return Err(anyhow!("The question_before doesn't link to the question\nquestion_before: `{:?}`\nquestion: `{:?}`", question_before, old_animal));
    };

    if was_yes {
        update_question_by_id(
            connection,
            question_before.id,
            None,
            Some(new_question.id),
            None
        )?;
    } else {
        update_question_by_id(
            connection,
            question_before.id,
            None,
            None,
            Some(new_question.id)
        )?;
    }

    Ok(())
}