use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, Serialize, Deserialize)]
pub enum MaelstromErrorCode {
    Timeout = 0,
    NodeNotFound = 1,
    NotSupported = 10,
    TemporarilyUnavailable = 11,
    MalformedRequest = 12,
    Crash = 13,
    Abort = 14,
    KeyDoesNotExist = 20,
    KeyAlreadyExists = 21,
    PreconditionFailed = 22,
    TxnConflict = 30,
}

impl Display for MaelstromErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let error_description = match self {
            MaelstromErrorCode::Timeout => {
                "The requested operation got timed out"
            },
            MaelstromErrorCode::NodeNotFound => {
                "The node which you sent the message to does not exist."
            },
            MaelstromErrorCode::NotSupported => {
                "The requested operation is not supported by the current implementation."
            },
            MaelstromErrorCode::TemporarilyUnavailable => {
                "The operation definitely cannot be performed at this time."
            },
            MaelstromErrorCode::MalformedRequest => {
                "The request did not conform to the server's expectations."
            },
            MaelstromErrorCode::Crash => {
                "A general, indefinite error occurred."
            },
            MaelstromErrorCode::Abort => {
                "The operation definitely did not take place."
            },
            MaelstromErrorCode::KeyDoesNotExist => {
                "The key in which you requested an operation does not exist."
            },
            MaelstromErrorCode::KeyAlreadyExists => {
                "The key already exists, and the server will not overwrite it."
            },
            MaelstromErrorCode::PreconditionFailed => {
                "The requested operation expected some conditions to hold, and those conditions were not met."
            },
            MaelstromErrorCode::TxnConflict => {
                "The requested transaction has been aborted due to a conflict with another transaction."
            },
        };
        write!(f, "{}", error_description)
    }
}

pub struct MaelstromError {
    r#type: String,
    in_reply_to: i32,
    code: MaelstromErrorCode,
    text: String,
}

impl MaelstromError {
    /// TODO: Modificar para receber como primeiro parâmetro uma request
    pub fn new(in_reply_to: i32, code: MaelstromErrorCode) -> Self {
        MaelstromError {
            r#type: "error".to_string(),
            text: code.to_string(),
            in_reply_to,
            code,
        }
    }
}
