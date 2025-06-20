use crate::query::parser::QueryRequest;
use crate::query::executor::execute_query;


use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::net::TcpListener;
use tokio::io::AsyncWriteExt;
use tokio::io::AsyncBufReadExt;
use anyhow::Result;
use serde_json::from_str;
use tracing::{info, debug, warn, error, trace};

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
        info!("Server started, listening on {}", self.addr);

        loop {
            let (stream, _) = listener.accept().await?;
            let storage = Arc::clone(&self.storage);

            info!("Accepted connection from {}", stream.peer_addr().unwrap());

            tokio::spawn(async move {
                if let Err(e) = handle_client(stream, storage).await {
                    error!("Client error: {}", e);
                }
            });
        }
    }
}

// Handle incoming connections
async fn handle_client(
    stream: tokio::net::TcpStream,
    storage: Arc<tokio::sync::Mutex<StorageEngine>>,
) -> Result<()> {
    let (reader, mut writer) = stream.into_split();
    let mut reader = tokio::io::BufReader::new(reader);

    loop {
        let mut line = String::new();
        let bytes_read = reader.read_line(&mut line).await?;
        if bytes_read == 0 {
            println!("Client disconnected");
            break;
        }
        let line = line.trim();
        println!("Received: {}", line);

        // Parse JSON query request
        let query: QueryRequest = match serde_json::from_str(line) {
            Ok(q) => q,
            Err(e) => {
                println!("ERROR trying to parse JSON from request");
                let err_msg = format!("Error parsing DataPoint: {}\n", e);
                writer.write_all(err_msg.as_bytes()).await?;
                writer.flush().await?;
                continue;
            }
        };

        // Lock storage and execute
        let mut engine = storage.lock().await;
        let response = execute_query(&mut engine, query);

        // Write JSON response
        let response_text = serde_json::to_string(&response)?;
        writer.write_all(response_text.as_bytes()).await?;
        writer.write_all(b"\n").await?;
        writer.flush().await?;
    }

    Ok(())
}