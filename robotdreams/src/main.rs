use std::env;
use slug::slugify;
use std::error::Error;

fn lowercase(input: &str) -> Result<String, Box<dyn Error>> {
    Ok(input.to_lowercase())
}

fn uppercase(input: &str) -> Result<String, Box<dyn Error>> {
    Ok(input.to_uppercase())
}

fn no_spaces(input: &str) -> Result<String, Box<dyn Error>> {
    Ok(input.replace(" ", ""))
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("No argument provided, please use (lowercase, uppercase, no-spaces, or slugify)");
        return Ok(());
    }

    let mut input_text = String::new();

    println!("Please enter the text you want to transform:");
    std::io::stdin().read_line(&mut input_text)?;

    let input_text = input_text.trim();

    let result = match args[1].as_str() {
        "lowercase" => format!("Lowercase: {}", lowercase(input_text)?),
        "uppercase" => format!("Uppercase: {}", uppercase(input_text)?),
        "no-spaces" => format!("No Spaces: {}", no_spaces(input_text)?),
        "slugify" => format!("Slugify: {}", slugify(input_text)),
        _ => format!("Unknown action: {}", args[1]),
    };

    println!("{}", result);
    Ok(())
}
