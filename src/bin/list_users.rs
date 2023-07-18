use diesel::{QueryDsl, RunQueryDsl, SelectableHelper};
use login_api::{establish_connection, models::User, schema::users::dsl::*};

fn main() {
    let connection = &mut establish_connection();
    let results = users
        .select(User::as_select())
        .load(connection)
        .expect("Error loading users");

    println!("{:?}", results);
}
