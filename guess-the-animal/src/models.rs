use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::questions)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Question {
    pub id: i32,
    pub question: String,
    // pub yes_id: i32,
    // pub no_id: i32,
}

/*#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::animals)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Animal {
    pub id: i32,
    pub name: String,
}*/

/*#[derive(Insertable)]
#[diesel(table_name = questions)]
pub struct NewQuestion {
    pub question: String,
    pub yes_id: Option<i32>,
    pub no_id: Option<i32>,
}

#[derive(Insertable)]
#[table_name = animals]
pub struct NewAnimal {
    pub name: String,
}*/