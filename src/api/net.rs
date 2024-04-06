//! `Net` namespace
use crate::prelude::*;
use crate::{api::Namespace, error::Result, types::U256, Transport};

/// `Net` namespace
#[derive(Debug, Clone)]
pub struct Net<T> {
    transport: T,
}

impl<T: Transport> Namespace<T> for Net<T> {
    fn new(transport: T) -> Self
    where
        Self: Sized,
    {
        Net { transport }
    }

    fn transport(&self) -> &T {
        &self.transport
    }
}

impl<T: Transport> Net<T> {
    /// Returns the network id.
    pub async fn version(&self) -> Result<String> {
        self.transport.execute("net_version", vec![]).await
    }

    /// Returns number of peers connected to node.
    pub async fn peer_count(&self) -> Result<U256> {
        self.transport.execute("net_peerCount", vec![]).await
    }

    /// Whether the node is listening for network connections
    pub async fn is_listening(&self) -> Result<bool> {
        self.transport.execute("net_listening", vec![]).await
    }
}

#[cfg(test)]
mod tests {
    use super::Net;
    use crate::{api::Namespace, rpc::Value, types::U256};

    rpc_test! (
      Net:version => "net_version";
      Value::String("Test123".into()) => "Test123"
    );

    rpc_test! (
      Net:peer_count => "net_peerCount";
      Value::String("0x123".into()) => U256::from(0x123)
    );

    rpc_test! (
      Net:is_listening => "net_listening";
      Value::Bool(true) => true
    );
}
