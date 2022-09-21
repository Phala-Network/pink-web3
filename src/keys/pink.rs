//! impl Key for Pink env

use core::convert::TryInto;

use crate::{
    signing::{Key, Signature, SigningError},
    types::Address,
};
use ethereum_types::H256;
use pink::chain_extension::{signing, SigType};

/// The ECDSA keypair using Pink's signing API
pub struct KeyPair {
    privkey: [u8; 32],
    address: Address,
}

impl From<[u8; 32]> for KeyPair {
    fn from(privkey: [u8; 32]) -> Self {
        let pubkey: [u8; 33] = signing::get_public_key(&privkey, SigType::Ecdsa)
            .try_into()
            .expect("Public key should be of length 33");
        let mut address = [0u8; 20];
        ink_env::ecdsa_to_eth_address(&pubkey, &mut address).expect("Get address of ecdsa failed");
        Self {
            privkey,
            address: address.into(),
        }
    }
}

impl KeyPair {
    /// Derive a new keypair from current pink contract's private key
    pub fn derive_keypair(salt: &[u8]) -> Self {
        let privkey_sr25519 = signing::derive_sr25519_key(salt);
        let privkey: [u8; 32] = privkey_sr25519[0..32]
            .try_into()
            .expect("Derive returned an invalid key");
        privkey.into()
    }
}

impl Key for KeyPair {
    fn sign(&self, message: &[u8; 32], chain_id: Option<u64>) -> Result<Signature, SigningError> {
        let signature = signing::ecdsa_sign_prehashed(&self.privkey, *message);
        let recovery_id: u64 = signature[64].into();
        let standard_v = recovery_id;
        let v = if let Some(chain_id) = chain_id {
            // When signing with a chain ID, add chain replay protection.
            standard_v + 35 + chain_id * 2
        } else {
            // Otherwise, convert to 'Electrum' notation.
            standard_v + 27
        };
        let r = H256::from_slice(&signature[..32]);
        let s = H256::from_slice(&signature[32..64]);

        Ok(Signature { v, r, s })
    }

    fn sign_message(&self, message: &[u8; 32]) -> Result<Signature, SigningError> {
        let signature = signing::ecdsa_sign_prehashed(&self.privkey, *message);
        let recovery_id: u64 = signature[64].into();

        let v = recovery_id;
        let r = H256::from_slice(&signature[..32]);
        let s = H256::from_slice(&signature[32..64]);

        Ok(Signature { v, r, s })
    }

    fn address(&self) -> Address {
        self.address
    }
}

impl Key for &KeyPair {
    fn sign(&self, message: &[u8; 32], chain_id: Option<u64>) -> Result<Signature, SigningError> {
        (*self).sign(message, chain_id)
    }

    fn sign_message(&self, message: &[u8; 32]) -> Result<Signature, SigningError> {
        (*self).sign_message(message)
    }

    fn address(&self) -> Address {
        (*self).address()
    }
}
