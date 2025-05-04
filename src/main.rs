use whatsandra::{
    Event, WhatsAppError,
    client::{Client, ClientConfig, LogLevel},
};
use dotenv::dotenv;
use env_logger;
use tokio::signal;

mod storage;
use storage::{SQLiteStorage, StorageError};

// Define a more comprehensive error type for our application
#[derive(Debug)]
enum AppError {
    WhatsAppError(()),
    StorageError(()),
}

impl From<WhatsAppError> for AppError {
    fn from(_: WhatsAppError) -> Self {
        AppError::WhatsAppError(())
    }
}

impl From<StorageError> for AppError {
    fn from(_: StorageError) -> Self {
        AppError::StorageError(())
    }
}

#[tokio::main]
async fn main() -> Result<(), AppError> {
    // Load .env file if available
    dotenv().ok();

    // Initialize env_logger
    env_logger::init();

    // Initialize storage
    let storage = SQLiteStorage::new()?;
    storage.connect().await?;
    println!("✅ SQLite storage initialized");

    // Create a configuration for WhatsApp client
    let config = ClientConfig {
        store_path: "whatsapp_store".to_string(),
        log_level: LogLevel::Debug,
    };

    // Create the client
    let client = Client::new(config);
    let _client_clone = client.clone();

    // Add event handler with storage capability
    let storage_clone = storage;
    client.add_event_handler(move |event| {
        let storage = storage_clone.clone();
        match event {
            Event::Connected => {
                println!("✅ Connected to WhatsApp!");
            },
            Event::Disconnected => {
                println!("❌ Disconnected from WhatsApp");
            },
            Event::QRCodeGenerated(qr) => {
                println!("🔐 Scan this QR code with your WhatsApp app:");
                println!("{}", qr);
            },
            Event::LoggedIn(jid) => {
                println!("🎉 Logged in as {}", jid);
            },
            Event::LoggedOut => {
                println!("👋 Logged out");
            },
            Event::MessageReceived(msg) => {
                if let Some(text) = &msg.text {
                    println!("📩 Received message: {}", text);

                    // Store message in SQLite
                    let msg_copy = msg.clone();
                    tokio::spawn(async move {
                        if let Err(e) = storage.store_message(&msg_copy).await {
                            eprintln!("Failed to store message: {}", e);
                        }
                    });
                }
            },
            _ => {
                // Ignore other events
            }
        }
    });

    // Connect to WhatsApp
    println!("Connecting to WhatsApp...");
    client.connect()?;

    // Wait for Ctrl+C signal
    println!("WhatsApp client running. Press Ctrl+C to exit");
    signal::ctrl_c().await.expect("Failed to listen for Ctrl+C");

    println!("Shutting down gracefully...");

    // Disconnect would be handled by Drop trait, but you could add explicit disconnect here
    // client.disconnect().await?;

    Ok(())
}
