use robotdreams::text_utils;
use robotdreams::csv_utils;
use std::env;
use std::error::Error;
use std::io;
use flume::{Receiver, Sender};

fn main() -> Result<(), Box<dyn Error>> {
    // Collect command-line arguments.
    let args: Vec<String> = env::args().collect();

    // Check if the user provided the expected number of arguments.
    if args.len() < 2 {
        eprintln!("No argument provided, please use (lowercase, uppercase, no-spaces, csv, or slugify)");
        return Ok(());
    }

    // Create a channel for sending input to the processing thread.
    let (tx, rx): (Sender<String>, Receiver<String>) = flume::unbounded();

    let input_thread = std::thread::spawn(move || {
        // Read input lines until an empty line is encountered.
        let mut input_text = String::new();

        // Create a table, treats the first row as the header, the rest as rows. Double enter (an empty line indicates the end of the operation)
        loop {
            let mut line = String::new();
            io::stdin().read_line(&mut line).unwrap();
            if line.trim().is_empty() {
                break;
            }
            input_text.push_str(&line);
        }
        // Send the input to the processing thread.
        tx.send(input_text).unwrap();
    });

    // Spin up the processing thread.
    let processing_thread = std::thread::spawn(move || {
        // Receive input from the input thread.
        let input_text = rx.recv().unwrap();
        match args[1].as_str() {
            "lowercase" => {
                println!("Lowercase: {}", text_utils::lowercase(&input_text).unwrap());
            }
            "uppercase" => {
                println!("Uppercase: {}", text_utils::uppercase(&input_text).unwrap());
            }
            "no-spaces" => {
                println!("No Spaces: {}", text_utils::no_spaces(&input_text).unwrap());
            }
            "csv" => {
                csv_utils::process_csv(&input_text).unwrap();
            }
            "slugify" => {
                println!("Slugify: {}", slug::slugify(&input_text));
            }
            _ => {
                eprintln!("Unknown action: {}", args[1]);
            }
        }
    });

    // Wait for both threads to finish.
    input_thread.join().unwrap();
    processing_thread.join().unwrap();

    Ok(())
}