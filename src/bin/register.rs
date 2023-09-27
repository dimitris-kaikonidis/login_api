use login_api::establish_connection;
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
}
