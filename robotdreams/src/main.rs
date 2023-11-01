use robotdreams::text_utils;
use robotdreams::csv_utils;
use std::env;
use std::error::Error;
use std::io;

fn main() -> Result<(), Box<dyn Error>> {
    // Collect command-line arguments.
    let args: Vec<String> = env::args().collect();

    // Check if the user provided the expected number of arguments.
    if args.len() < 2 {
        eprintln!("No argument provided, please use (lowercase, uppercase, no-spaces, csv, capitalize, reverse or slugify)");
        return Ok(());
    }

    // Initialize an empty string to collect user input.
    let mut input_text = String::new();

    println!("Double enter (an empty line indicates the end of the operation. For csv argument only: Create a table, treats the first row as the header, the rest as rows. ");

    // Read input lines until an empty line is encountered.
    loop {
        let mut line = String::new();
        io::stdin().read_line(&mut line)?;

        if line.trim().is_empty() {
            break;
        }

        input_text.push_str(&line);
    }

    let input_text = input_text.trim();

    // Execute the requested operation based on the provided argument.
    match args[1].as_str() {
    "lowercase" => {
        println!("Lowercase: {}", text_utils::lowercase(input_text)?);
    }
    "uppercase" => {
        println!("Uppercase: {}", text_utils::uppercase(input_text)?);
    }
    "no-spaces" => {
        println!("No Spaces: {}", text_utils::no_spaces(input_text)?);
    }
    "reverse" => {
        println!("Reverse: {}", text_utils::reverse(input_text)?);
    }
    "capitalize" => {
        println!("Capitalize: {}", text_utils::capitalize(input_text)?);
    }
    "csv" => {
        // Process the CSV input.
        csv_utils::process_csv(input_text)?;
    }
    "slugify" => {
        println!("Slugify: {}", slug::slugify(input_text));
    }
    _ => {
        eprintln!("Unknown action: {}", args[1]);
    }
}


    Ok(())
}