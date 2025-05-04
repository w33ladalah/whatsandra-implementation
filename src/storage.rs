use whatsandra::message::Message;
use std::error::Error;
use std::fmt;
use std::env;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use rusqlite::Connection;

#[derive(Debug)]
pub enum StorageError {
    DatabaseError(String),
}

impl fmt::Display for StorageError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            StorageError::DatabaseError(msg) => write!(f, "Database error: {}", msg),
        }
    }
}

impl Error for StorageError {}

#[derive(Clone)]
pub struct SQLiteStorage {
    db_path: PathBuf,
    // Using Arc<Mutex<>> to make the connection shareable across threads
    // In a real app, consider using a connection pool
    connection: Option<Arc<Mutex<Connection>>>,
}

impl SQLiteStorage {
    pub fn new() -> Result<Self, StorageError> {
        // Read configuration from environment
        let db_path = env::var("SQLITE_DB_PATH")
            .unwrap_or_else(|_| "whatsapp_messages.db".to_string());

        if db_path.is_empty() {
            return Err(StorageError::DatabaseError("Database path cannot be empty".to_string()));
        }

        Ok(Self {
            db_path: PathBuf::from(db_path),
            connection: None,
        })
    }

    pub async fn connect(&mut self) -> Result<(), StorageError> {
        println!("Connecting to SQLite database at: {}", self.db_path.display());

        // Open the database connection
        let conn = Connection::open(&self.db_path)
            .map_err(|e| StorageError::DatabaseError(format!("Failed to open database: {}", e)))?;

        // Create the messages table if it doesn't exist
        conn.execute(
            "CREATE TABLE IF NOT EXISTS messages (
                id TEXT PRIMARY KEY,
                from_me INTEGER,
                timestamp TEXT,
                message_type TEXT,
                chat_jid TEXT,
                text TEXT
            )",
            [],
        ).map_err(|e| StorageError::DatabaseError(format!("Failed to create table: {}", e)))?;

        // Store the connection
        self.connection = Some(Arc::new(Mutex::new(conn)));
        println!("✅ Successfully connected to SQLite database");

        Ok(())
    }

    pub async fn store_message(&self, msg: &Message) -> Result<(), StorageError> {
        let text = msg.text.as_deref().unwrap_or("[No text]");
        println!("Storing message in SQLite: {}", text);

        // Get the connection
        let conn = match &self.connection {
            Some(conn) => conn.clone(),
            None => return Err(StorageError::DatabaseError("Database not connected".to_string())),
        };

        // Lock the connection and execute the query
        let conn = conn.lock().map_err(|e|
            StorageError::DatabaseError(format!("Failed to lock connection: {}", e)))?;

        // Convert message type to a string for storage
        let message_type = format!("{:?}", msg.message_type);

        // Convert chat_jid to string
        let chat_jid = msg.chat_jid.to_string();

        conn.execute(
            "INSERT INTO messages (id, from_me, timestamp, message_type, chat_jid, text)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            rusqlite::params![
                msg.id,
                msg.from_me as i32,
                msg.timestamp.to_string(),
                message_type,
                chat_jid,
                text,
            ],
        ).map_err(|e| StorageError::DatabaseError(format!("Failed to insert message: {}", e)))?;

        println!("✅ Message successfully stored in SQLite database");
        Ok(())
    }
}
