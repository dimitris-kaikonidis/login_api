use crate::schema::users;
use diesel::{pg::Pg, prelude::Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};

#[derive(Debug, Queryable, Selectable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(Pg))]
pub struct User {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub display_name: Option<String>,
    pub email: String,
    pub password: String,
}
