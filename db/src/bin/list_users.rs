use db::{establish_connection, models::User, schema::users::dsl::users};
use diesel::{QueryDsl, RunQueryDsl, SelectableHelper};

pub(crate) fn main() {
    let connection = &mut establish_connection();
    let results = users
        .select(User::as_select())
        .load(connection)
        .expect("Error loading users");

    println!("{:?}", results);
}
