# WhatsAndra Implementation

This project demonstrates how to use the WhatsAndra Rust library to create a WhatsApp client that can connect to WhatsApp and store messages in SQLite.

## Prerequisites

- Rust and Cargo
- Internet connection for WhatsApp Web

## Setup

1. Clone this repository
2. Create a `.env` file in the root directory (optional)

Example `.env` file:

```dotenv
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
- Store received messages in SQLite database

## Project Structure

- `src/main.rs` - Main application entry point
- `src/storage.rs` - SQLite storage module

## Database Schema

The application creates an SQLite database with the following schema:

```sql
CREATE TABLE IF NOT EXISTS messages (
    id TEXT PRIMARY KEY,
    from_me INTEGER,
    timestamp TEXT,
    message_type TEXT,
    chat_jid TEXT,
    text TEXT
);
```

## Troubleshooting

If you encounter WebSocket connection errors like:

```bash
WebSocketError: WebSocket HTTP error: Invalid HTTP version specified
```

Try these solutions:

1. Check your internet connection
2. Ensure you're not behind a restrictive proxy or firewall
3. Set more detailed logging with `RUST_LOG=debug,whatsandra=trace cargo run`

## License

This project is open source and available under the MIT License.
