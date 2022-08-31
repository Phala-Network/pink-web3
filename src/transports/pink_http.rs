//! Pink HTTP transport
//!
//! This transport lets you use the library inside pink contract using pink's http API

use core::{
    future::{ready, Future, Ready},
    pin::Pin,
    task,
};
use serde::de::DeserializeOwned;

use crate::helpers::CallFuture;
use crate::prelude::*;
use crate::{error::TransportError, helpers::json_rpc};
use crate::{Error, Transport};

/// A Transport using pink http API
///
/// # Example
/// ```rust
/// fn get_web3_sha3() {
///     use pink_web3::api::Web3;
///     use pink_web3::transports::pink_http::PinkHttpTransport;
///     let phttp = PinkHttpTransport::<1024>::new("http://localhost:3333");
///     let web3 = Web3::new(phttp);
///     let result = web3.sha3(b"123".to_vec().into()).resolve();
///     assert!(result.is_ok());
/// }
/// ```
#[derive(Clone)]
pub struct PinkHttpTransport<const BUFLEN: usize> {
    url: String,
}

impl<const N: usize> PinkHttpTransport<N> {
    /// Create a new PinkHttpTransport instance
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

type RpcResult = Result<Vec<u8>, Error>;

impl<const N: usize> Transport for PinkHttpTransport<N> {
    type Out = Ready<RpcResult>;

    fn execute(&self, method: &'static str, params: Vec<&dyn erased_serde::Serialize>) -> Self::Out {
        let request = json_rpc::encode_request::<_, N>(method, params);
        let body = request.as_bytes();
        let headers: Vec<(String, String)> = vec![("Content-Type".into(), "application/json".into())];
        let response = pink::http_post!(&self.url, body, headers);
        if response.status_code / 100 != 2 {
            return ready(Err(Error::Transport(TransportError::Code(response.status_code))));
        }
        ready(Ok(response.body))
    }
}

impl<T: DeserializeOwned> CallFuture<T, Ready<RpcResult>> {
    /// Blocking resolves the output
    pub fn resolve(self) -> <Self as Future>::Output {
        resolve_ready(self)
    }
}

/// Retreive the output of a Future driven by PinkHttpTransport
///
/// When using PinkHttpTransport as the transport, the Futures returned by any API should be always
/// ready immediate because of pink's blocking HTTP api.
pub fn resolve_ready<F: Future>(fut: F) -> <F as Future>::Output {
    let waker = futures::task::noop_waker_ref();
    let mut cx = task::Context::from_waker(waker);
    use task::Poll::*;
    pin_mut!(fut);
    match fut.poll(&mut cx) {
        Ready(v) => v,
        Pending => panic!("Failed to resolve a ready future"),
    }
}
