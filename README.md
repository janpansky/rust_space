# Rust course project

Rust developer course by Braiins <3 - https://robot-dreams-rust.mag.wiki/intro.html.

## Project Structure (branch networking)

The project is organized into three crates:

1. **client**: The client-side application responsible for sending messages and files to the server.
2. **server**: The server-side application that listens for incoming messages and handles file transfers.
3. **shared_library**: A shared library containing message types used by both the client and server.

## Usage

1. Clone the repository:
   ```bash
   git clone git@github.com:janpansky/rust_space.git
   cd robotdreams
3. Build the project:
    ```bash
    cargo build --release
4. Run the server:
   ```bash
   cargo run --release --bin server
5. Run the client:
   ```bash
   cargo run --release --bin client
   
### Crates Conversion
The project has been converted into separate crates for client, server, and a shared library for message types. The workspace is defined in the Cargo.toml at the project root.