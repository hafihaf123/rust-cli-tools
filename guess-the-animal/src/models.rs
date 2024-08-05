use diesel::prelude::*;
use crate::schema::{questions, animals};

#[derive(Queryable, Selectable)]
#[diesel(table_name = questions)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Question {
    pub id: i32,
    pub question: String,
    pub yes_id: Option<i32>,
    pub no_id: Option<i32>,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = animals)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Animal {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable)]
#[diesel(table_name = questions)]
pub struct NewQuestion<'a> {
    pub question: &'a str,
    // pub yes_id: Option<i32>,
    // pub no_id: Option<i32>,
}

#[derive(Insertable)]
#[diesel(table_name = animals)]
pub struct NewAnimal<'a> {
    pub name: &'a str,
}