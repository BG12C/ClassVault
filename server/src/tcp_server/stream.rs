use crate::{
    event_handler::EventHandler,
    types::{
        client::Client,
        protocols::{ClientProtocol, ServerProtocol},
        user::User,
    },
    util::util::{check_username, write_to_stream},
};
use std::{
    collections::HashMap,
    io::Read,
    net::{SocketAddr, TcpListener, TcpStream},
    sync::{Arc, Mutex},
};

const BUFFER_SIZE: usize = 2048;

pub struct Server {
    pub connected_clients: Arc<Mutex<HashMap<String, (TcpStream, Client)>>>,
    pub tcp_listener: TcpListener,
}

impl Server {
    pub fn create(endpoint: SocketAddr) -> std::io::Result<Server> {
        let connected_clients = Arc::new(Mutex::new(HashMap::new()));
        let tcp_listener = TcpListener::bind(endpoint)?;
        log::info!("Server started @ {:#?}", endpoint);

        for stream in tcp_listener.incoming() {
            match stream {
                Ok(stream) => {
                    let connected_clients = connected_clients.clone();
                    log::info!("{} connected,", stream.peer_addr()?);

                    // Each client get's a custom thread
                    tokio::spawn(async move {
                        let mut current_client: Option<(String, String)> = None;

                        {
                            let connected_clients = connected_clients.clone();

                            // We need the HWID here so we can identify the client
                            while current_client.is_none() {
                                log::info!("Waiting for HWID...");
                                if let Some(c) = EventHandler::handle_auth(&stream) {
                                    current_client = Some(c);
                                }
                            }

                            // TODO: Check if HWID already exists, if not create entry with UUID
                            let client_some = current_client.clone().unwrap();
                            let session_token = uuid::Uuid::new_v4().to_string();
                            let username = check_username(&client_some.1);

                            let client = Client {
                                session_token: session_token.clone(),
                                hwid: client_some.0.clone(),
                                user: User { name: username },
                            };
                            log::info!("Found Hwid [{}]", client_some.0);

                            let message = ServerProtocol::AuthenticateToken {
                                token: &session_token,
                            };
                            write_to_stream(&stream, &message);

                            connected_clients
                                .lock()
                                .expect("Can't lock clients")
                                .insert(client_some.0, (stream.try_clone().unwrap(), client));

                            log::info!("{:#?}", connected_clients.lock().unwrap());

                            Self::handle_connection(&stream, &connected_clients);
                        }

                        // This will trigger after the client is disconnected & removes them from the HashMap
                        let mut connected_locked =
                            connected_clients.lock().expect("Unable to lock variable");
                        connected_locked.remove(&current_client.clone().unwrap().0);

                        log::info!("Client disconnected");
                        log::info!("{:#?}", connected_locked);
                    });
                    // We do not join the threads because then only one connections works at a time!
                }
                Err(why) => {
                    log::error!("Error accepting client connection");
                    log::error!("{}", why);
                }
            }
        }

        Ok(Server {
            connected_clients,
            tcp_listener,
        })
    }

    fn handle_connection(
        mut stream: &TcpStream,
        clients: &Arc<Mutex<HashMap<String, (TcpStream, Client)>>>,
    ) {
        let mut buffer = [0; BUFFER_SIZE];

        loop {
            match stream.read(&mut buffer) {
                Ok(bytes_read) => {
                    if bytes_read == 0 {
                        log::info!("Client disconnected");
                        break;
                    }

                    let incoming_message =
                        String::from_utf8_lossy(&buffer[0..bytes_read]).to_string();

                    log::info!("{incoming_message}");

                    let deserialized_message: Result<ClientProtocol, _> =
                        serde_json::from_str(&incoming_message);

                    match deserialized_message {
                        Ok(client_message) => match client_message {
                            ClientProtocol::SendMessage { hwid, content } => {
                                EventHandler::handle_send_message(&hwid, &content, clients);
                            }

                            // Every other message
                            _ => EventHandler::handle_unknown_message(&client_message),
                        },
                        Err(why) => {
                            log::error!("Error parsing client message, {}", why);
                        }
                    }

                    // Clear the buffer
                    buffer = [0; BUFFER_SIZE];
                }
                Err(why) => {
                    log::error!("{}", why);
                    break;
                }
            }
        }
    }
}
