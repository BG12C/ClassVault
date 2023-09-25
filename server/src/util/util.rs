use std::{io::Write, net::TcpStream};

pub fn write_to_stream<T>(mut stream: &TcpStream, content: &T)
where
    T: serde::Deserialize<'static> + serde::Serialize, // struct T must have trait Serialize & Deserialize
{
    let serialized_message = serde_json::to_string(&content).expect("Serialization failed");

    if stream.write_all(serialized_message.as_bytes()).is_err() {
        log::warn!("[❌] There was an error broadcasting the message");
    } else {
        log::info!("[✔] Message broadcasted!");
    }
}

pub fn check_username(name: &str) -> String {
    if name.len() < 1 || name.len() > 32 || !is_alphanumeric_with_symbols(&name) {
        return format!("User{}", rand::prelude::random::<i16>());
    }

    return name.to_string();
}

fn is_alphanumeric_with_symbols(input: &str) -> bool {
    for c in input.chars() {
        if !c.is_alphanumeric() && !c.is_ascii_punctuation() {
            return false;
        }
    }
    true
}
