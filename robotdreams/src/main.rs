mod csv_utils;
mod text_utils;

use std::error::Error;
use std::thread;
use flume::{Receiver, Sender};
use std::str::FromStr;
use std::fs;
use std::env;

#[derive(Debug)]
enum Command {
    Lowercase,
    Uppercase,
    NoSpaces,
    Csv,
    Slugify,
}

impl FromStr for Command {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "lowercase" => Ok(Command::Lowercase),
            "uppercase" => Ok(Command::Uppercase),
            "no-spaces" => Ok(Command::NoSpaces),
            "csv" => Ok(Command::Csv),
            "slugify" => Ok(Command::Slugify),
            _ => Err(()),
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // Create channels for communication between threads.
    let (input_sender, input_receiver) = flume::unbounded::<String>();

    let (result_sender, result_receiver) = flume::unbounded::<String>();

    // Spawn an input-receiving thread.
    let input_thread = thread::spawn(move || {
        loop {
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            input_sender.send(input).unwrap();
        }
    });

    // Spawn a processing thread.
    let processing_thread = thread::spawn(move || {
        loop {
            let input = input_receiver.recv().unwrap();
            let parts: Vec<&str> = input.trim().splitn(2, ' ').collect();
            if parts.is_empty() {
                continue; // Ignore empty lines
            }

            let command = parts[0];

            match Command::from_str(command) {
                Ok(Command::Lowercase) => {
                    let text = parts.get(1).unwrap_or(&"").to_string();
                    let result = text.to_lowercase();
                    result_sender.send(result).unwrap();
                }
                Ok(Command::Uppercase) => {
                    let text = parts.get(1).unwrap_or(&"").to_string();
                    let result = text.to_uppercase();
                    result_sender.send(result).unwrap();
                }
                Ok(Command::NoSpaces) => {
                    let text = parts.get(1).unwrap_or(&"").to_string();
                    let result = text.replace(" ", "");
                    result_sender.send(result).unwrap();
                }
                Ok(Command::Csv) => {
                    if parts.len() < 2 {
                        eprintln!("Missing CSV filename. Use: csv <filename>");
                        continue;
                    }
                    let filename = parts[1];

                    // Get the current working directory and construct the full path to the CSV file
                    let current_dir = env::current_dir().expect("Something is wrong with the path");
                    let csv_path = current_dir.join(filename);
                    // println!("path {:?} and {:?}", current_dir, csv_path);

                    // Use {csv input.csv} command
                    match read_csv_file(&csv_path) {
                        Ok(csv_data) => {
                            // Automatically parse and process the CSV data here
                            // For example, you can process the CSV data and send the result
                            // result_sender.send(processed_csv_data).unwrap();
                            println!("CSV Data:\n{:?}", csv_utils::process_csv(csv_data));
                        }
                        Err(err) => eprintln!("Error reading CSV file: {}", err),
                    }
                }
                Ok(Command::Slugify) => {
                    let text = parts.get(1).unwrap_or(&"").to_string();
                    let result = slug::slugify(text);
                    result_sender.send(result).unwrap();
                }
                Err(_) => {
                    eprintln!("Unknown command: {}", command);
                }
            }
        }
    });

    // Display results
    for result in result_receiver.iter() {
        println!("Result: {}", result);
    }

    Ok(())
}

fn read_csv_file(filename: &std::path::Path) -> Result<String, Box<dyn Error>> {
    let contents = fs::read_to_string(filename)?;
    Ok(contents)
}