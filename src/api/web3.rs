//! `Web3` namespace

use crate::{
    helpers::{self, CallFuture},
    types::{Bytes, DeString, H256},
    Transport,
};
use crate::Eth;

/// `Web3` namespace
#[derive(Debug, Clone)]
pub struct Web3<T> {
    transport: T,
}

impl<T: Transport + Clone> Web3<T> {
    /// Create a new instance
    pub fn new(transport: T) -> Self
    where
        Self: Sized,
    {
        Web3 { transport }
    }

    /// Create an ETH api for the instance
    pub fn eth(&self) -> Eth<T> {
        Eth::new(self.transport.clone())
    }
}

impl<T: Transport> Web3<T> {
    /// Returns client version
    pub fn client_version(&self) -> CallFuture<DeString, T::Out> {
        CallFuture::new(self.transport.execute("web3_clientVersion", vec![]))
    }

    /// Returns sha3 of the given data
    pub fn sha3(&self, bytes: Bytes) -> CallFuture<H256, T::Out> {
        let bytes = helpers::serialize(&bytes);
        CallFuture::new(self.transport.execute("web3_sha3", vec![bytes]))
    }
}

#[cfg(test)]
mod tests {
    use super::Web3;
    use crate::{transports::pink_http, types::H256};
    use hex_literal::hex;

    rpc_test! (
      Web3:client_version => "web3_clientVersion";
      "\"Test123\"" => "Test123"
    );

    rpc_test! (
      Web3:sha3, hex!("01020304")
      =>
      "web3_sha3", r#"["0x01020304"]"#;
      "\"0x0000000000000000000000000000000000000000000000000000000000000123\"" => H256::from_low_u64_be(0x123)
    );

    #[test]
    #[ignore = "for dev"]
    fn test_pink_api() {
        pink_extension_runtime::mock_ext::mock_all_ext();

        let phttp = pink_http::PinkHttpTransport::<1024>::new("http://localhost:3333");
        let web3 = Web3::new(phttp);
        let result = web3.sha3(b"123".to_vec().into()).resolve();
        assert!(result.is_ok());
    }
}
