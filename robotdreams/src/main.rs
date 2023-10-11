use std::env;
use std::io;
use slug::slugify;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("No argument provided, please use (lowercase, uppercase, no-spaces or slugify)");
        return;
    }

    let mut input_text = String::new();

    println!("Please enter the text you want to transform:");

    io::stdin()
        .read_line(&mut input_text)
        .expect("Failed to read line");

    // Shadow the variable
    let input_text = input_text.trim();

    // Iterate over arguments provided, skip the first
    for action in args.iter().skip(1) {
        match action.as_str() {
            "lowercase" => {
                println!("Lowercase: {}", input_text.to_lowercase());
            }
            "uppercase" => {
                println!("Uppercase: {}", input_text.to_uppercase());
            }
            "no-spaces" => {
                println!("No Spaces: {}", input_text.replace(" ", ""));
            }
            "slugify" => {
                println!("Slugify: {}", slugify(input_text));
            }
            _ => {
                println!("Unknown action: {}", action);
            }
        }
    }
}