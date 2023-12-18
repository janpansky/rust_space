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
   ```
2. Build the project:
    ```bash
   cargo build --release
   ```
3. Run the server:
   ```bash
   cd robotdreams
   cargo run --release --bin server
   ```
4. Run the client:
   ```bash
   cd robotdreams
   cargo run --release --bin client
   ```
5. Sending a Text message:

In the client terminal, enter a message and press Enter. The message will be sent to the server, and you should see the
server log the received text.

6. Sending a File:
   To send a file, type the following command in the client terminal:
   ```
   .file rust.txt
   ```

7. Sending an Image (test the conversion as well with jpg):
   ```
   .image rust.png
   or
   .image rust.jpg
   ```

### Crates Conversion

The project has been converted into separate crates for client, server, and a shared library for message types. The
workspace is defined in the Cargo.toml at the project root.

### Additional Features

The server uses SQLite for storing client information and text messages.

A login mechanism is implemented, authenticating clients based on constants **USER** and **PASSWORD**. Every user has
the same credentials at the moment, but unique identification as CLIENT-IP is stored in the db.

#### Access SQLite data

when building from the project root, otherwise, put one more dot if in server folder.

```
export DATABASE_URL=sqlite:../database.sqlite
```

```
sqlite3 database.sqlite
SELECT * FROM users;
or
SELECT * FROM chat_messages;
.exit
```

# Actix Integration

Run the server at http://127.0.0.1:8080/

```bash
   cargo run --bin robotdreams
   ```

## API Endpoints

### Users

- **GET /users**: Get a list of all users.

### Chat Messages

- **GET /chat_messages**: Get a list of all chat messages.

## API Testing

To test the API endpoints, you can use tools like `curl` or use a web browser. Below are examples using `curl`:

### Get Users

```bash
curl http://127.0.0.1:8080/users
```

### Get Chat messages

```bash
curl http://127.0.0.1:8080/chat_messages
```

# Web server Integration

Run the server at http://127.0.0.1:8080/

```bash
   cargo run --bin web_server
   ```