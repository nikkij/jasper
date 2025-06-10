use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::net::TcpListener;
use tokio::io::AsyncWriteExt;
use tokio::io::AsyncBufReadExt;
use anyhow::Result;
use serde_json::from_str;

use crate::storage::engine::StorageEngine;
use crate::types::DataPoint;

pub struct TCPServer {
    addr: String,
    storage: Arc<Mutex<StorageEngine>>,
}

impl TCPServer {
    pub fn new(addr: String, storage: Arc<Mutex<StorageEngine>>) -> Self {
        Self { addr, storage }
    }

    pub async fn run(&self) -> Result<()> {
        let listener = TcpListener::bind(&self.addr).await?;
        println!("Server listening on {}", self.addr);

        loop {
            let (stream, _) = listener.accept().await?;
            let storage = Arc::clone(&self.storage);

            println!("Accepted connection from {}", stream.peer_addr().unwrap());

            tokio::spawn(async move {
                if let Err(e) = handle_client(stream, storage).await {
                    eprintln!("Client error: {}", e);
                }
            });
        }
    }
}

// Handle incoming connections
async fn handle_client(
    mut stream: tokio::net::TcpStream,
    storage: Arc<tokio::sync::Mutex<StorageEngine>>,
) -> Result<()> {
    
    let (reader, mut writer) = stream.split();
    let mut reader = tokio::io::BufReader::new(reader);
    let mut writer = tokio::io::BufWriter::new(writer);

    loop {
        let mut line = String::new();
        let bytes_read = reader.read_line(&mut line).await?;

        if bytes_read == 0 {
            println!("Client disconnected");
            break;
        }

        let line = line.trim();
        println!("Received: {}", line);

        if line.starts_with("write ") {
            let json_str = &line[6..];
            let dp: DataPoint = match serde_json::from_str(json_str) {
                Ok(dp) => dp,
                Err(e) => {
                    let err_msg = format!("Error parsing DataPoint: {}\n", e);
                    writer.write_all(err_msg.as_bytes()).await?;
                    writer.flush().await?;
                    continue;
                }
            };
            // Write to storage
            let mut engine = storage.lock().await;
            if let Err(e) = engine.write(&dp) {
                let err_msg = format!("Error writing DataPoint: {}\n", e);
                writer.write_all(err_msg.as_bytes()).await?;
                writer.flush().await?;
                continue;
            }
            writer.write_all(b"OK\n").await?;
            writer.flush().await?;

        } else if line == "read" {
            let engine = storage.lock().await;
            // Read all DataPoints from storage
            match engine.read_all() {
                Ok(contents) => {
                    // Send the contents back
                    // You can send raw JSON or format nicely; let's send JSON lines:
                    for dp in contents {
                        let json_line = serde_json::to_string(&dp)? + "\n";
                        writer.write_all(json_line.as_bytes()).await?;
                    }
                    writer.flush().await?;
                }
                Err(e) => {
                    let err_msg = format!("Error reading storage: {}\n", e);
                    writer.write_all(err_msg.as_bytes()).await?;
                    writer.flush().await?;
                }
            }
        } else {
            writer.write_all(b"Unknown command\n").await?;
            writer.flush().await?;
        }
    }

    Ok(())
}