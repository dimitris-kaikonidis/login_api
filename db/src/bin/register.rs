use db::{establish_connection, models::User, schema::users::dsl::users};
use diesel::{insert_into, RunQueryDsl};
use rpassword::prompt_password;
use text_io::read;

fn main() {
    let connection = &mut establish_connection();

    print!("Enter email: ");
    let email: String = read!();
    let password = prompt_password("Enter password: ").expect("Could not process input.");

    loop {
        let password_confirm: String =
            prompt_password("Confirm password: ").expect("Could not process input.");

        if password == password_confirm {
            break;
        };
    }

    print!("Enter first name (optional): ");
    let first_name: String = read!();

    print!("Enter last name (optional): ");
    let last_name: String = read!();

    print!("Enter display name (optional): ");
    let display_name: String = read!();

    let user = User {
        first_name: Some(first_name),
        last_name: Some(last_name),
        display_name: Some(display_name),
        email,
        password,
    };

    insert_into(users)
        .values(user)
        .execute(connection)
        .expect("Registration failed.");
}
