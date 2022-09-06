//! Signing capabilities and utilities.
use crate::prelude::*;
use crate::types::H256;

/// Error during signing.
#[derive(Debug, derive_more::Display, PartialEq, Clone)]
pub enum SigningError {
    /// A message to sign is invalid. Has to be a non-zero 32-bytes slice.
    #[display(fmt = "Message has to be a non-zero 32-bytes slice.")]
    InvalidMessage,
}
#[cfg(feature = "std")]
impl std::error::Error for SigningError {}

/// Error during sender recovery.
#[derive(Debug, derive_more::Display, PartialEq, Clone)]
pub enum RecoveryError {
    /// A message to recover is invalid. Has to be a non-zero 32-bytes slice.
    #[display(fmt = "Message has to be a non-zero 32-bytes slice.")]
    InvalidMessage,
    /// A signature is invalid and the sender could not be recovered.
    #[display(fmt = "Signature is invalid (check recovery id).")]
    InvalidSignature,
}
#[cfg(feature = "std")]
impl std::error::Error for RecoveryError {}

#[cfg(feature = "signing")]
pub use feature_gated::*;

#[cfg(feature = "signing")]
mod feature_gated {
    use super::*;
    use crate::types::Address;
    /// A trait representing ethereum-compatible key with signing capabilities.
    ///
    /// The purpose of this trait is to prevent leaking `secp256k1::SecretKey` struct
    /// in stack or memory.
    /// To use secret keys securely, they should be wrapped in a struct that prevents
    /// leaving copies in memory (both when it's moved or dropped). Please take a look
    /// at:
    /// - https://github.com/graphprotocol/solidity-bindgen/blob/master/solidity-bindgen/src/secrets.rs
    /// - or https://crates.io/crates/zeroize
    /// if you care enough about your secrets to be used securely.
    ///
    /// If it's enough to pass a reference to `SecretKey` (lifetimes) than you can use `SecretKeyRef`
    /// wrapper.
    pub trait Key {
        /// Sign given message and include chain-id replay protection.
        ///
        /// When a chain ID is provided, the `Signature`'s V-value will have chain replay
        /// protection added (as per EIP-155). Otherwise, the V-value will be in
        /// 'Electrum' notation.
        fn sign(&self, message: &[u8], chain_id: Option<u64>) -> Result<Signature, SigningError>;

        /// Sign given message without manipulating V-value; used for typed transactions
        /// (AccessList and EIP-1559)
        fn sign_message(&self, message: &[u8]) -> Result<Signature, SigningError>;

        /// Get public address that this key represents.
        fn address(&self) -> Address;
    }
}

/// A struct that represents the components of a secp256k1 signature.
pub struct Signature {
    /// V component in electrum format with chain-id replay protection.
    pub v: u64,
    /// R component of the signature.
    pub r: H256,
    /// S component of the signature.
    pub s: H256,
}

/// Compute the Keccak-256 hash of input bytes.
pub fn keccak256(bytes: &[u8]) -> [u8; 32] {
    use tiny_keccak::{Hasher, Keccak};
    let mut output = [0u8; 32];
    let mut hasher = Keccak::v256();
    hasher.update(bytes);
    hasher.finalize(&mut output);
    output
}

/// Result of the name hash algotithm.
pub type NameHash = [u8; 32];

/// Compute the hash of a domain name using the namehash algorithm.
///
/// [Specification](https://docs.ens.domains/contract-api-reference/name-processing#hashing-names)
pub fn namehash(name: &str) -> NameHash {
    let mut node = [0u8; 32];

    if name.is_empty() {
        return node;
    }

    let mut labels: Vec<&str> = name.split('.').collect();

    labels.reverse();

    for label in labels.iter() {
        let label_hash = keccak256(label.as_bytes());

        node = keccak256(&[node, label_hash].concat());
    }

    node
}

/// Hash a message according to EIP-191.
///
/// The data is a UTF-8 encoded string and will enveloped as follows:
/// `"\x19Ethereum Signed Message:\n" + message.length + message` and hashed
/// using keccak256.
pub fn hash_message<S>(message: S) -> H256
where
    S: AsRef<[u8]>,
{
    let message = message.as_ref();

    let mut eth_message = format!("\x19Ethereum Signed Message:\n{}", message.len()).into_bytes();
    eth_message.extend_from_slice(message);

    keccak256(&eth_message).into()
}

#[cfg(test)]
mod tests {
    use super::*;

    //See -> https://eips.ethereum.org/EIPS/eip-137 for test cases

    #[test]
    fn name_hash_empty() {
        let input = "";

        let result = namehash(input);

        let expected = [0u8; 32];

        assert_eq!(expected, result);
    }

    #[test]
    fn name_hash_eth() {
        let input = "eth";

        let result = namehash(input);

        let expected = [
            0x93, 0xcd, 0xeb, 0x70, 0x8b, 0x75, 0x45, 0xdc, 0x66, 0x8e, 0xb9, 0x28, 0x01, 0x76, 0x16, 0x9d, 0x1c, 0x33,
            0xcf, 0xd8, 0xed, 0x6f, 0x04, 0x69, 0x0a, 0x0b, 0xcc, 0x88, 0xa9, 0x3f, 0xc4, 0xae,
        ];

        assert_eq!(expected, result);
    }

    #[test]
    fn name_hash_foo_eth() {
        let input = "foo.eth";

        let result = namehash(input);

        let expected = [
            0xde, 0x9b, 0x09, 0xfd, 0x7c, 0x5f, 0x90, 0x1e, 0x23, 0xa3, 0xf1, 0x9f, 0xec, 0xc5, 0x48, 0x28, 0xe9, 0xc8,
            0x48, 0x53, 0x98, 0x01, 0xe8, 0x65, 0x91, 0xbd, 0x98, 0x01, 0xb0, 0x19, 0xf8, 0x4f,
        ];

        assert_eq!(expected, result);
    }
}
