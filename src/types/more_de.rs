use crate::prelude::*;

use core::ops::Deref;
use serde::Deserialize;

/// A string wrapper to be deserializable with serde_json_core
#[derive(Debug, PartialEq, Eq)]
pub struct DeString(String);

impl<'de> Deserialize<'de> for DeString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s: &str = Deserialize::deserialize(deserializer)?;
        Ok(Self(s.into()))
    }
}

impl From<&str> for DeString {
    fn from(s: &str) -> Self {
        Self(s.to_owned())
    }
}

impl From<String> for DeString {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl From<DeString> for String {
    fn from(s: DeString) -> Self {
        s.0
    }
}

impl Deref for DeString {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}