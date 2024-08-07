use anyhow::{Context, Result};
use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper};
use diesel::SqliteConnection;

use crate::diesel_files::models::{Animal, NewAnimal};
use crate::schema::animals::dsl::*;

pub fn get_all_animals() -> Result<Vec<Animal>> {
    let connection = &mut crate::establish_connection()?;
    animals
        .limit(5)
        .select(Animal::as_select())
        .load(connection)
        .with_context(|| "Could not load animals")
}

pub fn get_animal_by_id(id_: i32) -> Result<Option<Animal>> {
    let connection = &mut crate::establish_connection()?;

    animals
        .find(id_)
        .select(Animal::as_select())
        .first(connection)
        .optional()
        .with_context(|| format!("Could not load animal with id `{}`", id_))
}

pub fn create_animal(connection: &mut SqliteConnection, animal: Animal) -> Result<Animal> {
    use crate::schema::animals;

    let new_animal = NewAnimal { name: &*animal.name };

    diesel::insert_into(animals::table)
        .values(&new_animal)
        .returning(Animal::as_returning())
        .get_result(connection)
        .with_context(|| "Could not save the animal")
}

pub fn delete_animal_by_id(connection: &mut SqliteConnection, n: i32) -> Result<usize> {
    diesel::delete(animals.filter(id.eq(n)))
        .execute(connection)
        .with_context(|| format!("Could not delete animal with id `{}`", n))
}

pub fn delete_animal_by_name(connection: &mut SqliteConnection, name_: &str) -> Result<usize> {
    diesel::delete(animals.filter(name.eq(name_)))
        .execute(connection)
        .with_context(|| format!("Could not delete animal with name `{}`", name_))
}

pub fn update_animal_name(connection: &mut SqliteConnection, id_: i32, name_: &str) -> Result<Animal> {
    diesel::update(animals.find(id_))
        .set(name.eq(name_))
        .returning(Animal::as_returning())
        .get_result(connection)
        .with_context(|| format!("Could not update name to Animal with id `{}`", id_))
}

#[cfg(test)]
mod tests {
    use crate::establish_connection;

    use super::*;

    fn init_animals() -> Vec<Animal> {
        let mut res = Vec::new();
        res.push(Animal {
            id: 1,
            name: "dog?".to_string(),
        });
        res.push(Animal {
            id: 2,
            name: "cat!".to_string(),
        });
        res.push(Animal {
            id: 3,
            name: "Mouse()".to_string(),
        });
        res.push(Animal {
            id: 4,
            name: "cat!".to_string(),
        });
        res
    }

    fn setup_tests() {
        let mut conn = establish_connection().unwrap();
        for animal in init_animals() {
            create_animal(&mut conn, animal).unwrap();
        }
    }

    fn cleanup_tests_by_id() -> usize {
        let mut conn = establish_connection().unwrap();
        let mut deleted_num = delete_animal_by_id(&mut conn, 4).unwrap();
        deleted_num += delete_animal_by_id(&mut conn, 3).unwrap();
        deleted_num += delete_animal_by_id(&mut conn, 2).unwrap();
        deleted_num += delete_animal_by_id(&mut conn, 1).unwrap();
        deleted_num
    }

    fn cleanup_tests_by_name() -> usize {
        let mut conn = establish_connection().unwrap();
        let mut deleted_num = 0;

        for animal in init_animals() {
            deleted_num += delete_animal_by_name(&mut conn, &animal.name).unwrap();
        }

        deleted_num
    }

    #[test]
    fn test_everything() {
        // test create_animals
        assert!(get_all_animals().unwrap().is_empty());
        setup_tests();

        test_get_all_animals();

        test_get_animal_by_id();

        // test delete_animal_by_id
        cleanup_tests_by_id();
        assert!(get_all_animals().unwrap().is_empty());
    }
    
    #[test]
    fn test_delete_animal_by_name() {
        setup_tests();
        test_get_all_animals();

        assert_eq!(cleanup_tests_by_name(), 4);
        assert!(get_all_animals().unwrap().is_empty());
    }

    fn test_get_all_animals() {
        let expected = init_animals();

        assert_eq!(get_all_animals().unwrap(), expected);
    }

    fn test_get_animal_by_id() {
        for animal in init_animals() {
            assert_eq!(get_animal_by_id(animal.id).unwrap().unwrap(), animal);
        }
        assert!(get_animal_by_id(5).unwrap().is_none());
        assert!(get_animal_by_id(0).unwrap().is_none());
        assert!(get_animal_by_id(-1).unwrap().is_none());
        assert!(get_animal_by_id(i32::MAX).unwrap().is_none());
        assert!(get_animal_by_id(i32::MIN).unwrap().is_none());
    }

    #[test]
    fn test_update_animal() {
        let mut conn = establish_connection().unwrap();

        setup_tests();
        test_get_all_animals();

        let expected_animal = Animal {
            id: 4,
            name: "cat!".to_string(),
        };
        assert_eq!(get_animal_by_id(4).unwrap().unwrap(), expected_animal);

        let expected_updated_animal = Animal {
            id: 4,
            name: "Mouse()".to_string(),
        };
        assert_eq!(update_animal_name(&mut conn, 4, "Mouse()").unwrap(), expected_updated_animal);

        cleanup_tests_by_id();
    }
}