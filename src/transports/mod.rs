//! Supported Ethereum JSON-RPC transports.

#[cfg(feature = "disabled")]
pub mod batch;

#[cfg(feature = "disabled")]
pub use self::batch::Batch;
#[cfg(feature = "disabled")]
pub mod either;
#[cfg(feature = "disabled")]
pub use self::either::Either;

#[cfg(any(feature = "http", feature = "http-rustls"))]
pub mod http;
#[cfg(any(feature = "http", feature = "http-rustls"))]
pub use self::http::Http;

#[cfg(any(feature = "ws-tokio", feature = "ws-async-std"))]
pub mod ws;
#[cfg(any(feature = "ws-tokio", feature = "ws-async-std"))]
pub use self::ws::WebSocket;

#[cfg(feature = "ipc-tokio")]
pub mod ipc;
#[cfg(feature = "ipc-tokio")]
pub use self::ipc::Ipc;

#[cfg(any(feature = "test", test))]
pub mod test;

#[cfg(feature = "url")]
impl From<url::ParseError> for crate::Error {
    fn from(err: url::ParseError) -> Self {
        use crate::error::TransportError;
        crate::Error::Transport(TransportError::Message(format!("failed to parse url: {}", err)))
    }
}

#[cfg(feature = "async-native-tls")]
impl From<async_native_tls::Error> for crate::Error {
    fn from(err: async_native_tls::Error) -> Self {
        use crate::error::TransportError;
        crate::Error::Transport(TransportError::Message(format!("{:?}", err)))
    }
}

#[cfg(feature = "eip-1193")]
pub mod eip_1193;

#[cfg(feature = "pink")]
pub mod pink_http;
#[cfg(feature = "pink")]
pub use pink_http::{resolve_ready, PinkHttp};
