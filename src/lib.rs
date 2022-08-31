//! Ethereum JSON-RPC client (Web3).

#![allow(
    clippy::type_complexity,
    clippy::wrong_self_convention,
    clippy::single_match,
    clippy::let_unit_value,
    clippy::match_wild_err_arm
)]
#![warn(missing_docs)]
// select! in WS transport
#![recursion_limit = "256"]
#![cfg_attr(not(any(feature = "std", feature = "test", test)), no_std)]

// it needs to be before other modules
// otherwise the macro for tests is not available.
#[macro_use]
pub mod helpers;

#[macro_use]
extern crate alloc;

mod prelude {
    pub(crate) use alloc::borrow::ToOwned as _;
    pub(crate) use alloc::string::String;
    pub(crate) use alloc::string::ToString as _;
    pub(crate) use alloc::vec::Vec;
    #[cfg(not(any(feature = "std", feature = "test", test)))]
    pub(crate) use core as std;
}

use prelude::*;

/// Re-export of the `futures` crate.
#[macro_use]
pub extern crate futures;

pub mod api;
pub mod error;
pub mod keys;
pub mod signing;
pub mod transports;
pub mod types;

pub use crate::{
    api::Eth,
    error::{Error, Result},
};

/// Assigned RequestId
pub type RequestId = usize;

// TODO [ToDr] The transport most likely don't need to be thread-safe.
// (though it has to be Send)
/// Transport implementation
pub trait Transport {
    /// The type of future this transport returns when a call is made.
    type Out: core::future::Future<Output = Result<Vec<u8>>>;

    /// Execute remote method with given parameters.
    fn execute(&self, method: &'static str, params: Vec<&dyn erased_serde::Serialize>) -> Self::Out;
}
