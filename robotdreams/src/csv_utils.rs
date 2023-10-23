use std::error::Error;
use csv::Reader;

pub fn process_csv(input: &str) -> Result<(), Box<dyn Error>> {
    let mut reader = Reader::from_reader(input.as_bytes());

    let headers = reader.headers()?.clone();
    let formatted_headers = headers.iter().collect::<Vec<&str>>().join(" | ");
    println!("{}", formatted_headers);

    for result in reader.records() {
        let record = result?;

        let formatted_record = record.iter().collect::<Vec<&str>>().join(" | ");
        println!("{}", formatted_record);
    }

    Ok(())
}