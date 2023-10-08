use std::io;
use std::env;
use slug::slugify;

fn main() {
    println!("Please enter your name:");

    let mut name = String::new();

    // Read user input into the 'name' variable
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");

    println!("Hello, {}! Welcome to the world of Rust!", name.trim());

    let args: Vec<String> = env::args().collect();

    println!("{}", args[0]);

    let name = name.trim();

    let lowercase_result = transform_text(name, "lowercase");
    println!("Lowercase: {}", lowercase_result);

    let uppercase_result = transform_text(name, "uppercase");
    println!("Uppercase: {}", uppercase_result);

    let no_spaces_result = transform_text(name, "no-spaces");
    println!("No Spaces: {}", no_spaces_result);

    let slugify_result = transform_text(name, "slugify");
    println!("Slugify: {}", slugify_result);

    let invalid_result = transform_text(name, "invalid-mode");
    println!("Invalid Mode: {}", invalid_result);
}

fn transform_text(text: &str, mode: &str) -> String {
    match mode {
        "lowercase" => text.to_lowercase(),
        "uppercase" => text.to_uppercase(),
        "no-spaces" => text.replace(" ", ""),
        "slugify" => slugify(text),
        _ => String::from("Invalid transformation mode"),
    }
}
