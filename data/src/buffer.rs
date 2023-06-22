use crate::user::Nick;
use crate::Server;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Buffer {
    Server(Server),
    Channel(Server, String),
    Query(Server, Nick),
}

impl Buffer {
    pub fn target(&self) -> Option<String> {
        match self {
            Buffer::Server(_) => None,
            Buffer::Channel(_, channel) => Some(channel.clone()),
            Buffer::Query(_, nick) => Some(nick.to_string()),
        }
    }
}