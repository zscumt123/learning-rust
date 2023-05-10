use crate::Value;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum KvError {
    #[error("Not found for table: {0}, key: {1}")]
    NotFound(String, String),

    #[error("Command is invalid: `{0}`")]
    InvalidCommand(String),
    #[error("Cannot convert value {0:?} to {1}")]
    ConvertError(Value, &'static str),
    #[error("Cannot process command {0} with table: {1}, key: {2}. Error: {3}")]
    StorageError(&'static str, String, String, String),

    #[error("sled error")]
    SledError(#[from] sled::Error),
    #[error("Failed to encode protobuf message")]
    EncodeError(#[from] prost::EncodeError),
    #[error("Failed to decode protobuf message")]
    DecodeError(#[from] prost::DecodeError),
    #[error("Frame error")]
    FrameError,
    #[error("io  error")]
    IoError(#[from] std::io::Error),
    #[error("cert error {0}: {1}")]
    CertifcateParseError(&'static str, &'static str),
    #[error("TLS error")]
    TlsError(#[from] tokio_rustls::rustls::TLSError),

    #[error("Internal error: {0}")]
    Internal(String),
}
