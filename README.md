# WhatsAndra Implementation

This project demonstrates how to use the WhatsAndra Rust library to create a WhatsApp client that can connect to WhatsApp and store messages in SQLite.

## Prerequisites

- Rust and Cargo
- Internet connection for WhatsApp Web

## Setup

1. Clone this repository
2. Create a `.env` file in the root directory (optional)

Example `.env` file:
```
# SQLite Configuration
SQLITE_DB_PATH=whatsapp_messages.db

# Logging
# Set to "info", "debug", "warn", "error", or "trace"
RUST_LOG=info
```

## Running the Application

```bash
cargo run
```

When you first run the application, it will generate a QR code. Scan this with your WhatsApp app to authenticate the connection.

## Features

- Connect to WhatsApp using QR code authentication
- Receive and display messages
- Ready for integration with SQLite for message storage

## Project Structure

- `src/main.rs` - Main application entry point
- `src/storage.rs` - SQLite storage module (skeleton implementation)

## Extending the Application

To add full SQLite storage functionality:

1. Implement the actual connection logic in `SQLiteStorage::connect()`
2. Implement the message storage logic in `SQLiteStorage::store_message()`
3. Uncomment the code in the `MessageReceived` event handler in `main.rs`

## License

This project is open source and available under the MIT License.
