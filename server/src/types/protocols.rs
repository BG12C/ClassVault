#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub enum ServerProtocol<'a> {
    BroadcastMessage { sender: &'a str, content: &'a str },
    AuthenticateToken { token: &'a str },
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub enum ClientProtocol<'a> {
    SendMessage {
        hwid: &'a str,
        content: &'a str,
    },

    /// This is the first message the clients sends when connecting to the server.
    RequestAuthentication {
        hwid: &'a str,
        name: &'a str,
    },
}

// Custom Protocol?
