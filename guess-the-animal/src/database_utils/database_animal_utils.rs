use diesel::SqliteConnection;
use crate::diesel_files::models::Animal;

pub fn get_animals() -> anyhow::Result<()> {
    use diesel::{QueryDsl, RunQueryDsl, SelectableHelper};
    use anyhow::Context;
    use crate::diesel_files::schema::animals::dsl::*;

    let connection = &mut crate::establish_connection()?;
    let results = animals
        .limit(5)
        .select(Animal::as_select())
        .load(connection)
        .with_context(|| "Could not load animals")?;

    println!("Displaying {} animals", results.len());
    for animal in results {
        println!("({}): {}", animal.id, animal.name);
    }
    Ok(())
}

pub fn get_animal_by_id(id: i32) -> anyhow::Result<()> {
    use diesel::{OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper};
    use anyhow::Context;
    use crate::diesel_files::models::Animal;
    use crate::diesel_files::schema::animals::dsl::animals;

    let connection = &mut crate::establish_connection()?;
    let animal = animals
        .find(id)
        .select(Animal::as_select())
        .first(connection)
        .optional()
        .with_context(|| format!("Could not load animal with id `{}`", id))?;

    match animal {
        Some(animal) => {
            println!("({}): {}", animal.id, animal.name);
        },
        None => {
            println!("There is no animal with id `{}`", id);
        },
    }
    Ok(())
}

pub fn create_animal(connection: &mut SqliteConnection, name: &str) -> anyhow::Result<Animal> {
    use diesel::{RunQueryDsl, SelectableHelper};
    use anyhow::Context;
    use crate::diesel_files::models::NewAnimal;
    use crate::diesel_files::schema::animals;

    let new_animal = NewAnimal { name };

    Ok(diesel::insert_into(animals::table)
        .values(&new_animal)
        .returning(Animal::as_returning())
        .get_result(connection)
        .with_context(|| "Could not save the animal")?)
}

pub fn delete_animal_by_id(connection: &mut SqliteConnection, n: i32) -> anyhow::Result<usize> {
    use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
    use anyhow::Context;
    use crate::diesel_files::schema::animals::dsl::*;

    Ok(diesel::delete(animals.filter(id.eq(n)))
        .execute(connection)
        .with_context(|| format!("Could not delete animal with id `{}`", n))?)
}

pub fn delete_animal_by_name(connection: &mut SqliteConnection, name_: String) -> anyhow::Result<usize> {
    use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
    use anyhow::Context;
    use crate::diesel_files::schema::animals::dsl::*;

    Ok(diesel::delete(animals.filter(name.eq(name_)))
        .execute(connection)
        .with_context(|| format!("Could not delete animal with name `{}`", name_))?)
}