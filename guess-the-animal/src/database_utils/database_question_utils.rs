use anyhow::{Context, Result};
use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper};
use diesel::SqliteConnection;

use crate::diesel_files::models::{NewQuestion, Question};
use crate::schema::questions::dsl::*;

pub fn get_all_questions() -> Result<Vec<Question>> {
    let connection = &mut crate::establish_connection()?;

    questions
        .limit(5)
        .select(Question::as_select())
        .load(connection)
        .with_context(|| "Could not load questions")
}

pub fn get_question_by_id(id_: i32) -> Result<Option<Question>> {
    let connection = &mut crate::establish_connection()?;
    questions
        .find(id_)
        .select(Question::as_select())
        .first(connection)
        .optional()
        .with_context(|| format!("Could not load question with id `{}`", id_))
}

pub fn create_question(connection: &mut SqliteConnection, question: Question) -> Result<Question> {
    use crate::schema::questions;

    let new_question = NewQuestion {
        content: &*question.content,
        yes_id: question.yes_id,
        no_id: question.no_id
    };

    diesel::insert_into(questions::table)
        .values(&new_question)
        .returning(Question::as_returning())
        .get_result(connection)
        .with_context(|| "Could not save the question")
}

pub fn delete_question_by_id(connection: &mut SqliteConnection, n: i32) -> Result<usize> {
    diesel::delete(questions.filter(id.eq(n)))
        .execute(connection)
        .with_context(|| format!("Could not delete question with id `{}`", n))
}

pub fn delete_question_by_content(connection: &mut SqliteConnection, content_: &str) -> Result<usize> {
    diesel::delete(questions.filter(content.eq(content_)))
        .execute(connection)
        .with_context(|| format!("Could not delete question with content `{}`", content_))
}

pub fn update_question_by_id(connection: &mut SqliteConnection, id_: i32, content_: Option<String>, yes_id_: Option<i32>, no_id_: Option<i32>) -> Result<Question> {
    let mut question: Question = Question {
        id: id_,
        content: "".to_string(),
        yes_id: None,
        no_id: None,
    };
    match content_ {
        None => {}
        Some(string) => {
            question = diesel::update(questions.find(id_))
                .set(content.eq(string))
                .returning(Question::as_returning())
                .get_result(connection)
                .with_context(|| format!("Could not update content to Question with id `{}`", id_))?;
        }
    }
    match yes_id_ {
        None => {}
        Some(num) => {
            question = diesel::update(questions.find(id_))
                .set(yes_id.eq(Some(num)))
                .returning(Question::as_returning())
                .get_result(connection)
                .with_context(|| format!("Could not update yes_id to Question with id `{}`", id_))?;
        }
    }
    match no_id_ {
        None => {}
        Some(num) => {
            question = diesel::update(questions.find(id_))
                .set(no_id.eq(num))
                .returning(Question::as_returning())
                .get_result(connection)
                .with_context(|| format!("Could not update no_id to Question with id `{}`", id_))?;
        }
    }

    Ok(question)
}

#[cfg(test)]
mod tests {
    use crate::establish_connection;

    use super::*;

    fn init_questions_with_relations() -> Vec<Question> {
        let mut res = Vec::new();
        res.push(Question {
            id: 1,
            content: "x y?".to_string(),
            yes_id: None,
            no_id: None,
        });
        res.push(Question {
            id: 2,
            content: "a b!".to_string(),
            yes_id: Some(1),
            no_id: None,
        });
        res.push(Question {
            id: 3,
            content: "o P()".to_string(),
            yes_id: None,
            no_id: Some(1),
        });
        res.push(Question {
            id: 4,
            content: "a b!".to_string(),
            yes_id: Some(2),
            no_id: Some(3),
        });
        res
    }

    fn init_questions_without_relations() -> Vec<Question> {
        let mut res = Vec::new();
        res.push(Question {
            id: 1,
            content: "x y?".to_string(),
            yes_id: None,
            no_id: None,
        });
        res.push(Question {
            id: 2,
            content: "a b!".to_string(),
            yes_id: None,
            no_id: None,
        });
        res.push(Question {
            id: 3,
            content: "o P()".to_string(),
            yes_id: None,
            no_id: None,
        });
        res.push(Question {
            id: 4,
            content: "a b!".to_string(),
            yes_id: None,
            no_id: None,
        });
        res
    }

    fn setup_tests() {
        let mut conn = establish_connection().unwrap();

        for question in init_questions_with_relations() {
            create_question(&mut conn, question).unwrap();
        }
    }

    fn setup_tests_without_relations() {
        let mut conn = establish_connection().unwrap();

        for question in init_questions_without_relations() {
            create_question(&mut conn, question).unwrap();
        }
    }

    fn cleanup_tests_by_id() -> usize {
        let mut conn = establish_connection().unwrap();
        let mut deleted_num = delete_question_by_id(&mut conn, 4).unwrap();
        deleted_num += delete_question_by_id(&mut conn, 3).unwrap();
        deleted_num += delete_question_by_id(&mut conn, 2).unwrap();
        deleted_num += delete_question_by_id(&mut conn, 1).unwrap();
        deleted_num
    }

    fn cleanup_tests_by_content() -> usize {
        let mut conn = establish_connection().unwrap();
        let mut deleted_num = 0;

        for question in init_questions_with_relations() {
            deleted_num += delete_question_by_content(&mut conn, &question.content).unwrap();
        }

        deleted_num
    }

    #[test]
    fn test_everything() {
        // test create_questions
        assert!(get_all_questions().unwrap().is_empty());
        setup_tests();

        // test get_questions
        test_get_all_questions();

        test_get_question_by_id();

        // test delete_question_by_id
        cleanup_tests_by_id() as i32;
        assert!(get_all_questions().unwrap().is_empty());
    }

    #[test]
    fn test_delete_question_by_content() {
        setup_tests_without_relations();
        test_get_all_questions_without_relations();

        assert_eq!(cleanup_tests_by_content(), 4);
        assert!(get_all_questions().unwrap().is_empty());
    }

    fn test_get_all_questions() {
        let expected = init_questions_with_relations();

        assert_eq!(get_all_questions().unwrap(), expected);
    }

    fn test_get_all_questions_without_relations() {
        let expected = init_questions_without_relations();

        assert_eq!(get_all_questions().unwrap(), expected);
    }

    fn test_get_question_by_id() {
        for question in init_questions_with_relations() {
            assert_eq!(get_question_by_id(question.id).unwrap().unwrap(), question);
        }
        assert!(get_question_by_id(5).unwrap().is_none());
        assert!(get_question_by_id(0).unwrap().is_none());
        assert!(get_question_by_id(-1).unwrap().is_none());
        assert!(get_question_by_id(i32::MAX).unwrap().is_none());
        assert!(get_question_by_id(i32::MIN).unwrap().is_none());
    }

    #[test]
    fn test_update_question_by_id() {
        let mut conn = establish_connection().unwrap();

        setup_tests();
        test_get_all_questions();

        let expected_question = Question {
            id: 4,
            content: "a b!".to_string(),
            yes_id: Some(2),
            no_id: Some(3),
        };
        assert_eq!(get_question_by_id(4).unwrap().unwrap(), expected_question);

        let expected_updated_question = Question {
            id: 4,
            content: "a b!".to_string(),
            yes_id: Some(2),
            no_id: Some(1),
        };
        assert_eq!(update_question_by_id(&mut conn, 4, None, None, Some(1)).unwrap(), expected_updated_question);

        cleanup_tests_by_id();
    }
}