use std::{
    collections::HashMap,
    io::Read,
    net::TcpStream,
    process,
    sync::{Arc, Mutex},
};

use crate::{
    types::{
        client::Client,
        protocols::{ClientProtocol, ServerProtocol},
    },
    util::util::write_to_stream,
};

pub struct EventHandler;

impl EventHandler {
    pub fn handle_send_message(
        hwid: &str,
        content: &str,
        clients: &Arc<Mutex<HashMap<String, (TcpStream, Client)>>>,
    ) {
        match clients.lock() {
            Ok(lock) => {
                for (client_hwid, (client_stream, c)) in &*lock {
                    if client_hwid.eq(&hwid) {
                        log::info!("{} --> {}", c.user.name, content.clone());

                        continue;
                    }

                    // The message which get's sent to everyone else
                    let message = ServerProtocol::BroadcastMessage {
                        sender: hwid,
                        content,
                    };

                    write_to_stream(client_stream, &message);
                }
            }
            Err(_) => {
                log::error!("There was an error locking the value");
            }
        }
    }

    pub fn handle_auth(mut stream: &TcpStream) -> Option<(String, String)> {
        // Doesn't need to be bigger since it's just the HWID Authorization event
        // let mut buffer = [0; 1024];
        let mut buffer = String::new();

        match stream.read_to_string(&mut buffer) {
            Ok(bytes_read) => {
                if bytes_read == 0 {
                    log::info!("Client disconnected");
                    process::exit(0);
                }

                // let incoming_message = String::from_utf8_lossy(&buffer[0..bytes_read]).to_string();
                let incoming_message = buffer.to_string();

                let deserialized_message: Result<ClientProtocol, _> =
                    serde_json::from_str(&incoming_message);

                match deserialized_message {
                    Ok(client_message) => {
                        if let ClientProtocol::RequestAuthentication { hwid, name } = client_message
                        {
                            buffer.clear();

                            Some((hwid.to_string(), name.to_string()))
                        } else {
                            log::error!("Received invalid event before authentication!");
                            buffer.clear();

                            None
                        }
                    }
                    Err(_) => None,
                }
            }

            Err(why) => {
                log::error!("Unable to read from stream! {why}");
                process::exit(0);
            }
        }
    }

    pub fn handle_unknown_message(client_message: &ClientProtocol) {
        log::warn!("Received unknown message {:#?}", client_message);
    }
}
