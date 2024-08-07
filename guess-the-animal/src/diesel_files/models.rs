use diesel::prelude::*;

use crate::schema::*;

#[derive(Queryable, Selectable, Debug, PartialEq)]
#[diesel(table_name = questions)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Question {
    pub id: i32,
    pub content: String,
    pub is_last: i32,
    pub yes_id: Option<i32>,
    pub no_id: Option<i32>,
}

impl Question {
    pub fn new(content: &str, yes_id: Option<i32>, no_id: Option<i32>) -> Question {
        Question {
            id: 0,
            content: content.to_string(),
            is_last: 0,
            yes_id,
            no_id,
        }
    }
    pub fn new_last(content: &str, yes_id: Option<i32>, no_id: Option<i32>) -> Question {
        Question {
            id: 0,
            content: content.to_string(),
            is_last: 1,
            yes_id,
            no_id,
        }
    }
    pub fn default() -> Question {
        Question {
            id: 0,
            content: "".to_string(),
            is_last: 0,
            yes_id: None,
            no_id: None,
        }
    }
}

#[derive(Insertable)]
#[diesel(table_name = questions)]
pub struct NewQuestion<'a> {
    pub content: &'a str,
    pub is_last: i32,
    pub yes_id: Option<i32>,
    pub no_id: Option<i32>,
}

impl NewQuestion<'_> {
    pub fn from_question(question: &Question) -> NewQuestion {
        NewQuestion {
            content: question.content.as_str(),
            is_last: question.is_last,
            yes_id: question.yes_id,
            no_id: question.no_id
        }
    }
}

#[derive(Queryable, Selectable, Debug, PartialEq)]
#[diesel(table_name = animals)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Animal {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable)]
#[diesel(table_name = animals)]
pub struct NewAnimal<'a> {
    pub name: &'a str,
}