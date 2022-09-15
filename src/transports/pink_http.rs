//! Pink HTTP transport
//!
//! This transport lets you use the library inside pink contract using pink's http API

use core::{
    future::{ready, Future, Ready},
    pin::Pin,
    task,
};
use serde::de::DeserializeOwned;

use crate::prelude::*;
use crate::{error::TransportError, helpers::json_rpc};
use crate::{Error, Transport};

/// A Transport using pink http API
///
/// # Example
/// ```rust
/// fn get_web3_sha3() {
///     use pink_web3::api::{Web3, Namespace};
///     use pink_web3::transports::pink_http::PinkHttp;
///     use pink_web3::Resolve;
///     let phttp = PinkHttp::new("http://localhost:3333");
///     let web3 = Web3::new(phttp);
///     let result = web3.web3().sha3(b"123".to_vec().into()).resolve();
///     assert!(result.is_ok());
/// }
/// ```
#[derive(Clone)]
pub struct PinkHttp {
    url: String,
}

impl PinkHttp {
    /// Create a new PinkHttp instance
    pub fn new(url: impl Into<String>) -> Self {
        Self { url: url.into() }
    }
}

struct Response(Result<Vec<u8>, Error>);

impl Future for Response {
    type Output = Result<Vec<u8>, Error>;

    fn poll(mut self: Pin<&mut Self>, _cx: &mut task::Context<'_>) -> task::Poll<Self::Output> {
        task::Poll::Ready(core::mem::replace(&mut self.0, Ok(vec![])))
    }
}

impl Transport for PinkHttp {
    type Out<T> = Ready<Result<T, Error>>;

    fn execute<T: DeserializeOwned>(&self, method: &'static str, params: Vec<crate::Value>) -> Self::Out<T> {
        let request = json_rpc::encode_request(method, params);
        let body = request.as_bytes();
        let headers: Vec<(String, String)> = vec![("Content-Type".into(), "application/json".into())];
        let response = pink::http_post!(&self.url, body, headers);
        if response.status_code / 100 != 2 {
            return ready(Err(Error::Transport(TransportError::Code(response.status_code))));
        }
        ready(json_rpc::decode_response(&response.body))
    }
}
