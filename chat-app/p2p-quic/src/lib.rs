mod connection;
mod endpoint;
mod hole_punching;
mod provider;
mod transport;

use std::net::SocketAddr;

pub use connection::{Connecting, Connection, Substream};
pub use endpoint::Config;
#[cfg(feature = "tokio")]
pub use provider::tokio;
pub use provider::Provider;
pub use transport::GenTransport;

// Errors
#[derive(Debug, thiserror::Error)]
pub mod Error {
    #[error(transparent)]
    Reach(#[from] ConnectError),
    
    #[error(transparent)]
    Connection(#[from] ConnectionError),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error("Endpoint driver crashed")]
    EndpointDriverCrashed,

    #[error("Handshake with the remote timed out.")]
    HandshakeTimeedOut,

    #[error("Tried to dial as listner without an active listner.")]
    NoActiveListnerForDialAsListner,

    #[error("Already punching hole for {0}.")]
    HolePunchInProgress(SockerAddr),
}

#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub struct ConnectError(#[from] quinn_proto::ConnectError);


#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub struct ConnectionError(#[from] quinn_proto::ConnectionError);
