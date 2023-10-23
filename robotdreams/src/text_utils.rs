use std::error::Error;

pub fn lowercase(input: &str) -> Result<String, Box<dyn Error>> {
    Ok(input.to_lowercase())
}

pub fn uppercase(input: &str) -> Result<String, Box<dyn Error>> {
    Ok(input.to_uppercase())
}

pub fn no_spaces(input: &str) -> Result<String, Box<dyn Error>> {
    Ok(input.replace(" ", ""))
}