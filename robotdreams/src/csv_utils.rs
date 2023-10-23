use csv::{Reader, StringRecord};
use std::error::Error;

pub fn process_csv(input: &str) -> Result<(), Box<dyn Error>> {
    let mut reader = Reader::from_reader(input.as_bytes());

    let headers = reader.headers()?.clone();

    let formatted_headers = format_csv_row(&headers, 16);
    println!("{}", formatted_headers);

    for result in reader.records() {
        let record = result?;

        let formatted_record = format_csv_row(&record, 16);
        println!("{}", formatted_record);
    }

    Ok(())
}

fn format_csv_row(row: &StringRecord, column_width: usize) -> String {
    row.iter()
        .map(|cell| {
            let truncated = if cell.len() <= column_width {
                cell.to_string()
            } else {
                cell.chars().take(column_width).collect::<String>()
            };
            format!("{:<width$}", truncated, width = column_width)
        })
        .collect::<Vec<String>>()
        .join(" | ")
}
