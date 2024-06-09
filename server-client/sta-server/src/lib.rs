//! This crate handles the server side actions for server-client app
//!
//! # Server hangling function:
//!
//! handle_client(mut stream: TcpStream, clients: Arc<Mutex<Vec<TcpStream>>>)
//!

use std::net::{TcpStream};
use std::io::{Read, Write};
use std::sync::{Arc, Mutex};
use log::{error, info};

pub fn handle_client(mut stream: TcpStream, clients: Arc<Mutex<Vec<TcpStream>>>) {

    /// Define size of buffer
    let mut buffer = [0; 512];

    /// Init the loop to detext connections
    loop {
        let addr = match stream.peer_addr() {
            Ok(addr) => addr,
            Err(e) => {
                error!("Error getting peer address: {}", e);
                break;
            }
        };
        match stream.read(&mut buffer) {
            Ok(0) => {
                /// Client disconnected
                info!("Client disconnected: {}", addr);

                /// Remove the client from the list
                let mut clients = clients.lock().unwrap();
                clients.retain(|client| {
                    match client.peer_addr() {
                        Ok(client_addr) => client_addr != addr,
                        Err(_) => true, // Keep the client if we can't get its add ress
                    }
                });

                break;
            }
            Ok(bytes_read) => {
                /// Broadcast the message to all other clients
                let mut clients = clients.lock().unwrap();
                for client in clients.iter_mut() {

                    let client_addr = match client.peer_addr() {
                        Ok(addr) => addr,
                        Err(_) => {
                            error!("Failed to get client address");
                            continue;
                        }
                    };

                    if client_addr == addr {
                        continue;
                    }

                    if let Err(e) = client.write_all(&buffer[..bytes_read]) {
                        error!("Failed to send data to client: {}", e);
                    } else {
                        let client_addr = match client.peer_addr() {
                            Ok(addr) => addr.to_string(),
                            Err(_) => String::from("Unknown"),
                        };
                        info!("Forwarded {} bytes to client: {}", bytes_read, client_addr);
                    }
                }
            }
            Err(e) => {
                error!("Error reading from stream: {}", e);
                break;
            }
        }
    }
}