use std::io::Write;

pub mod color;
pub mod indicator;
pub mod list;
pub mod open_link;
pub mod project;
pub mod refresh_user_token;

pub fn get_user_input(label: &str) -> String {
    print!("Enter {}: ", label);

    std::io::stdout().flush().expect("Failed to flush stdout");

    let mut input = String::new();

    std::io::stdin()
        .read_line(&mut input)
        .expect(&format!("Failed to read {}", label));

    let input = input.trim();

    input.to_string()
}

pub fn get_user_secure_input(label: &str) -> String {
    let secure_string = rpassword::prompt_password(&format!("Enter {}: ", label))
        .expect(&format!("Failed to read {}", label));
    secure_string
}
