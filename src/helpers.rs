//! Web3 helpers.

use crate::error;
use crate::prelude::*;

/// Takes any type which is deserializable from rpc::Value and such a value and
/// yields the deserialized value
pub fn decode<T: serde::de::DeserializeOwned>(value: Vec<u8>) -> error::Result<T> {
    json::from_slice(&value).map_err(Into::into)
}

/// Serialize a type. Panics if the type is returns error during serialization.
pub fn serialize<T: erased_serde::Serialize>(t: &T) -> &dyn erased_serde::Serialize {
    t as _
}

pub(crate) mod json_rpc {
    use crate::prelude::*;
    use crate::Error;

    use serde::{Deserialize, Serialize};

    #[derive(Serialize)]
    struct Request<'a, Params> {
        jsonrpc: &'a str,
        id: u32,
        method: &'a str,
        params: Params,
    }

    #[derive(Deserialize)]
    struct Response<T> {
        result: Option<T>,
        error: Option<RpcError>,
    }

    #[derive(Deserialize, Debug)]
    pub struct RpcError {
        pub code: i32,
        pub message: String,
    }

    pub fn encode_request<Params: Serialize>(method: &str, params: Params) -> String {
        json::to_string(&Request {
            jsonrpc: "2.0",
            id: 0,
            method,
            params,
        })
        .expect("Failed to encode rpc request")
        .to_string()
    }

    pub fn decode_response<'de, T: Deserialize<'de>>(response: &'de [u8]) -> Result<T, Error> {
        let response: Response<T> =
            json::from_slice(response).or(Err(Error::Decoder("Failed to decode the rpc response".into())))?;
        if let Some(result) = response.result {
            return Ok(result);
        }
        if let Some(error) = response.error {
            return Err(Error::Rpc(format!("{error:?}")));
        }
        if let Ok(result) = json::from_str("null") {
            return Ok(result);
        }
        Err(Error::Decoder("Invalid rpc response".into()))
    }
}

#[cfg(test)]
#[macro_use]
pub mod tests {
    macro_rules! rpc_test {
    // With parameters
    (
      $namespace: ident : $name: ident : $test_name: ident  $(, $param: expr)+ => $method: expr,  $results: expr;
      $returned: expr => $expected: expr
    ) => {
      #[test]
      fn $test_name() {
        // given
        let mut transport = $crate::transports::test::TestTransport::default();
        transport.set_response($returned);
        let eth = $namespace::new(&transport);
        let result = futures::executor::block_on(eth.$name($($param.into(), )+));
        // then
        transport.assert_request($method, &$results.into_iter().map(Into::into).collect::<Vec<_>>());
        transport.assert_no_more_requests();
        assert_eq!(result, Ok($expected.into()));
      }
    };
    // With parameters (implicit test name)
    (
      $namespace: ident : $name: ident $(, $param: expr)+ => $method: expr,  $results: expr;
      $returned: expr => $expected: expr
    ) => {
      rpc_test! (
        $namespace : $name : $name $(, $param)+ => $method, $results;
        $returned => $expected
      );
    };

    // No params entry point (explicit name)
    (
      $namespace: ident: $name: ident: $test_name: ident => $method: expr;
      $returned: expr => $expected: expr
    ) => {
      #[test]
      fn $test_name() {
        // given
        let mut transport = $crate::transports::test::TestTransport::default();
        transport.set_response($returned);
        let eth = $namespace::new(&transport);
        let result = futures::executor::block_on(eth.$name());
        // then
        transport.assert_request($method, &[]);
        transport.assert_no_more_requests();
        assert_eq!(result, Ok($expected.into()));
      }
    };

    // No params entry point
    (
      $namespace: ident: $name: ident => $method: expr;
      $returned: expr => $expected: expr
    ) => {
      rpc_test! (
        $namespace: $name: $name => $method;
        $returned => $expected
      );
    }
  }
}
