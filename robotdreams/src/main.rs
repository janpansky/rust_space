use robotdreams::text_utils;
use robotdreams::csv_utils;
use std::env;
use std::error::Error;
use std::io;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("No argument provided, please use (lowercase, uppercase, no-spaces, csv, or slugify)");
        return Ok(());
    }

    let mut input_text = String::new();

    println!("Create a table, treats first row as header, the rest as rows. Double enter (an empty line indicates the end of the operation");

    // Read input lines until an empty line is encountered
    loop {
        let mut line = String::new();
        io::stdin().read_line(&mut line)?;

        if line.trim().is_empty() {
            break;
        }

        input_text.push_str(&line);
    }

    let input_text = input_text.trim();

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
        "csv" => {
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