use std::error::Error;

/// Convert the input string to lowercase.
pub fn lowercase(input: &str) -> Result<String, Box<dyn Error>> {
    Ok(input.to_lowercase())
}

/// Convert the input string to uppercase.
pub fn uppercase(input: &str) -> Result<String, Box<dyn Error>> {
    Ok(input.to_uppercase())
}

/// Remove spaces from the input string.
pub fn no_spaces(input: &str) -> Result<String, Box<dyn Error>> {
    Ok(input.replace(" ", ""))
}

/// Reverse the input string.
pub fn reverse(input: &str) -> Result<String, Box<dyn Error>> {
    let reversed: String = input.chars().rev().collect();
    Ok(reversed)
}

/// Capitalize the input string.
pub fn capitalize(input: &str) -> Result<String, Box<dyn Error>> {
    let capitalized: String = input.to_uppercase();
    Ok(capitalized)
}
