use std::thread;
use std::error::Error;


mod lib;

fn main() -> Result<(), Box<dyn Error>> {
    let client_thread = thread::spawn(|| {
        if let Err(e) = client::main() {
            eprintln!("Client error: {}", e);
        }
    });

    client_thread.join().unwrap();

    Ok(())
}
