use std::sync::{Arc, Mutex};
use tokio::net::TcpListener;
use tokio::io::AsyncWriteExt;
use tokio::io::AsyncBufReadExt;
use anyhow::Result;
use crate::storage::engine::StorageEngine;

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
    storage: Arc<Mutex<StorageEngine>>,
) -> Result<()> {
    
    let (reader, mut writer) = stream.split();
    let mut reader = tokio::io::BufReader::new(reader);
    let mut writer = tokio::io::BufWriter::new(writer);

    loop {
        let mut line = String::new();
        let bytes_read = reader.read_line(&mut line).await?;

        if bytes_read == 0 {
            // Client closed connection
            println!("Client disconnected");
            break;
        }

        println!("Received: {}", line.trim());

        // For now, just send back a simple acknowledgment
        writer.write(b"OK\n").await?;
        writer.flush().await?;
    }

    Ok(())
}


// use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
// use tokio::net::{TcpListener, TcpStream};
// use anyhow::Result;

// pub struct TCPServer {
//     addr: String,
// }

// impl TCPServer {
//     pub fn new(addr: &str) -> Self {
//         TCPServer {
//             addr: addr.to_string(),
//         }
//     }

//     pub async fn run(&self) -> Result<()> {
//         let listener = TcpListener::bind(&self.addr).await?;
//         println!("Server listening on {}", self.addr);

//         loop {
//             let (socket, addr) = listener.accept().await?;
//             println!("Accepted connection from {}", addr);

//             // Spawn a new task to handle this client connection
//             tokio::spawn(async move {
//                 if let Err(e) = handle_client(socket).await {
//                     eprintln!("Error handling client {}: {:?}", addr, e);
//                 }
//             });
//         }
//     }
// }

// async fn handle_client(mut socket: TcpStream) -> Result<()> {
//     let (reader, mut writer) = socket.split();
//     let mut buf_reader = BufReader::new(reader);
//     let mut line = String::new();

//     loop {
//         line.clear();
//         let bytes_read = buf_reader.read_line(&mut line).await?;

//         if bytes_read == 0 {
//             // Client closed connection
//             println!("Client disconnected");
//             break;
//         }

//         println!("Received: {}", line.trim());

//         // For now, just send back a simple acknowledgment
//         writer.write_all(b"OK\n").await?;
//     }

//     Ok(())
// }

