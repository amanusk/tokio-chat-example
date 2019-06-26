//! tokio-chat-common provides the tokio codecs for client/server communication.
//!
//! This crate should be straightforward; see tokio-chat-server for a description of the
//! client/server protocol.

use serde::{Deserialize, Serialize};

use tokio_jsoncodec::Codec as JsonCodec;

// Handshake message sent from a client to a server when it first connects, identifying the
// username of the client.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Handshake {
    pub name: String,
}

impl Handshake {
    pub fn new<S: Into<String>>(name: S) -> Handshake {
        Handshake { name: name.into() }
    }
}

pub type HandshakeCodec = JsonCodec<Handshake, Handshake>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientMessage(pub String);

impl ClientMessage {
    pub fn new<S: Into<String>>(message: S) -> ClientMessage {
        ClientMessage(message.into())
    }
}

// Enumerate possible messages the server can send to clients.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServerMessage {
    // A message from a client (first String) containing arbitrary content (second String).
    Message(String, String),

    // Notification of a new user connection. The associated String is the name that user provided
    // in their Handshake.
    UserConnected(String),
    //
    // Notification of user disconnection. The associated String is the name that user provided
    // in their Handshake.
    UserDisconnected(String),
}

pub type ServerToClientCodec = JsonCodec<ClientMessage, ServerMessage>;
pub type ClientToServerCodec = JsonCodec<ServerMessage, ClientMessage>;
