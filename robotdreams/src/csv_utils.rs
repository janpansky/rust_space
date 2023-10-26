use csv::{Reader, StringRecord};
use std::error::Error;
use std::fmt;

const DEFAULT_COLUMN_WIDTH: usize = 16; // Define a constant for the default column width
const DEFAULT_FILLER: usize = 3; // Define a constant for the default filler value when size > 16


struct CsvRecord {
    data: StringRecord,
}

impl CsvRecord {
    fn new(data: StringRecord) -> Self {
        CsvRecord { data }
    }
}

// Example of handling more than 16 characters - my approach
// Input:
// name,surname,adress
// Jan,Pansky,Tahle adresa je moc dlouha az az
//
// Output:
// name             | surname          | adress
// Jan              | Pansky           | Tahle adresa ...
impl fmt::Display for CsvRecord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let column_width = DEFAULT_COLUMN_WIDTH;
        // Format each cell in the CSV row, truncating if necessary.
        let formatted_columns: Vec<String> = self
            .data
            .iter()
            .map(|cell| {
                let truncated = if cell.len() <= column_width {
                    cell.to_string()
                } else {
                    cell.chars()
                        .take(column_width - DEFAULT_FILLER)
                        .collect::<String>()
                        + "..."
                };
                format!("{:<width$}", truncated, width = column_width)
            })
            .collect();
        write!(f, "{}", formatted_columns.join(" | "))
    }
}

pub fn process_csv(input: String) -> Result<(), Box<dyn Error>> {
    let mut reader = Reader::from_reader(input.as_bytes());

    let headers = reader.headers()?.clone();

    let formatted_headers = CsvRecord::new(headers.clone());
    println!("{}", formatted_headers);

    for result in reader.records() {
        let record = match result {
            Ok(record) => CsvRecord::new(record),
            Err(err) => {
                eprintln!("Error reading record: {}", err);
                continue;
            }
        };

        // Check if the number of fields in the record matches the header.
        if record.data.len() != headers.len() {
            eprintln!("Record has a different number of fields. Skipping.");
            continue;
        }

        println!("{}", record);
    }

    Ok(())
}