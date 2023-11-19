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
   git clone -b networking git@github.com:janpansky/rust_space.git
   cd robotdreams
2. Build the project:
    ```bash
   cargo build --release
3. Run the server:
   ```bash
   cd robotdreams
   cargo run --release --bin server
4. Run the client:
   ```bash
   cd robotdreams
   cargo run --release --bin client
5. Sending a Text message:

In the client terminal, enter a message and press Enter. The message will be sent to the server, and you should see the server log the received text.

6. Sending a File:
To send a file, type the following command in the client terminal:
   ```bash
   .file filename.txt

7. Sending an Image:
   ```bash
   .image
   
### Crates Conversion
The project has been converted into separate crates for client, server, and a shared library for message types. The workspace is defined in the Cargo.toml at the project root.