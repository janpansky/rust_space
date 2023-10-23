use std::env;
use slug::slugify;
use std::error::Error;
use std::io;

fn lowercase(input: &str) -> Result<String, Box<dyn Error>> {
    Ok(input.to_lowercase())
}

fn uppercase(input: &str) -> Result<String, Box<dyn Error>> {
    Ok(input.to_uppercase())
}

fn no_spaces(input: &str) -> Result<String, Box<dyn Error>> {
    Ok(input.replace(" ", ""))
}

fn csv(input: &str) -> Result<(), Box<dyn Error>> {
    let mut reader = csv::Reader::from_reader(input.as_bytes());

    // Read the headers from the first line
    let headers = reader.headers()?.clone();

    // Join the headers into a pipe-separated string
    let formatted_headers = headers.iter().collect::<Vec<&str>>().join(" | ");
    println!("{}", formatted_headers);

    // Read and process data rows
    for result in reader.records() {
        let record = result?;

        // Join the fields of the record into a pipe-separated string
        let formatted_record = record.iter().collect::<Vec<&str>>().join(" | ");
        println!("{}", formatted_record);
    }

    Ok(())
}


fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("No argument provided, please use (lowercase, uppercase, no-spaces, csv, or slugify)");
        return Ok(());
    }

    let mut input_text = String::new();

    println!("Create a table, threats first row as header, the rest as rows. Double enter (an empty line indicates the end of the operation");

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
            println!("Lowercase: {}", lowercase(input_text)?);
        }
        "uppercase" => {
            println!("Uppercase: {}", uppercase(input_text)?);
        }
        "no-spaces" => {
            println!("No Spaces: {}", no_spaces(input_text)?);
        }
        "csv" => {
            csv(input_text)?;
        }
        "slugify" => {
            println!("Slugify: {}", slugify(input_text));
        }
        _ => {
            eprintln!("Unknown action: {}", args[1]);
        }
    }

    Ok(())
}
