use std::thread;
use std::error::Error;

mod lib;

fn main() -> Result<(), Box<dyn Error>> {
    let server_thread = thread::spawn(|| {
        if let Err(e) = lib::main() {
            eprintln!("Server error: {}", e);
        }
    });

    server_thread.join().unwrap();
    Ok(())
}