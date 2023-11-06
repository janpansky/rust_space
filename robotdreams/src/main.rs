use std::thread;
use std::error::Error;

mod server;
mod client;

fn main() -> Result<(), Box<dyn Error> > {
    let server_thread = thread::spawn(|| {
        if let Err(e) = server::main() {
            eprintln!("Server error: {}", e);
        }
    });

    let client_thread = thread::spawn(|| {
        if let Err(e) = client::main() {
            eprintln!("Client error: {}", e);
        }
    });

    server_thread.join().unwrap();
    client_thread.join().unwrap();

    Ok(())
}
