use std::io;

fn main() {
    println!("Please enter your name:");

    let mut name = String::new();

    // Read user input into the 'name' variable
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");

    println!("Hello, {}! Welcome to the world of Rust!", name.trim());
}