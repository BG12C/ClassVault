use super::user::User;

/// A client is a hidden "layer" of a user, which contains Hwid, Ip, etc.
#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq, Clone, Eq, Hash)]
pub struct Client {
    /// A custom name of the client
    pub hwid: String,
    /// A Session-Token is a randomly generated String which changes on every reconnect. It can be used to validate the session
    pub session_token: String,
    pub user: User,
}
