use crate::schema::{passwords, users};
use diesel::{pg::Pg, prelude::Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};

#[derive(Debug, Queryable, Selectable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(Pg))]
pub struct User {
    pub email: String,
    pub verifier: Vec<u8>,
    pub salt: Vec<u8>,
}

#[derive(Debug, Queryable, Selectable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = passwords)]
#[diesel(check_for_backend(Pg))]
pub struct Password {
    pub name: String,
    pub password: String,
}
