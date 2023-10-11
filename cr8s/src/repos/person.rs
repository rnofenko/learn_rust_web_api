use diesel::prelude::*;

use crate::{
    models::{NewPerson, Person},
    schema::persons,
};

pub struct PersonRepo;

impl PersonRepo {
    pub fn create(c: &mut PgConnection, new_entity: NewPerson) -> QueryResult<Person> {
        diesel::insert_into(persons::table)
            .values(new_entity)
            .get_result(c)
    }
}
