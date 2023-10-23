use csv::{Reader, StringRecord};
use std::error::Error;

pub fn process_csv(input: &str) -> Result<(), Box<dyn Error>> {
    // Create a CSV reader from the input.
    let mut reader = Reader::from_reader(input.as_bytes());

    // Read and clone the CSV header.
    let headers = reader.headers()?.clone();

    // Format and print the CSV header.
    let formatted_headers = format_csv_row(&headers, 16);
    println!("{}", formatted_headers);

    for result in reader.records() {
        let record = match result {
            Ok(record) => record,
            Err(err) => {
                eprintln!("Error reading record: {}", err);
                continue;
            }
        };

        // Check if the number of fields in the record matches the header.
        if record.len() != headers.len() {
            eprintln!("Record has a different number of fields. Skipping.");
            continue; // Skip this record and continue with the next one
        }

        // Format and print each CSV record.
        let formatted_record = format_csv_row(&record, 16);
        println!("{}", formatted_record);
    }

    Ok(())
}

// Example of handling more than 16 characters - my approach
// Input:
// name,surname,adress
// Jan,Pansky,Tahle adresa je moc dlouha az az
//
// Output:
// name             | surname          | adress
// Jan              | Pansky           | Tahle adresa ...
fn format_csv_row(row: &StringRecord, column_width: usize) -> String {
    row.iter()
        .map(|cell| {
            // Truncate cell text if it exceeds 16 characters and add "...".
            let truncated = if cell.len() <= column_width {
                cell.to_string()
            } else {
                cell.chars().take(column_width - 3).collect::<String>() + "..."
            };
            format!("{:<width$}", truncated, width = column_width)
        })
        .collect::<Vec<String>>()
        .join(" | ")
}