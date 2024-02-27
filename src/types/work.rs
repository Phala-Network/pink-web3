use crate::types::{H256, U256};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Miner's work package
#[derive(Debug, PartialEq, Eq)]
pub struct Work {
    /// The proof-of-work hash.
    pub pow_hash: H256,
    /// The seed hash.
    pub seed_hash: H256,
    /// The target.
    pub target: H256,
    /// The block number: this isn't always stored.
    pub number: Option<u64>,
}

impl<'a> Deserialize<'a> for Work {
    fn deserialize<D>(deserializer: D) -> Result<Work, D::Error>
    where
        D: Deserializer<'a>,
    {
        let (pow_hash, seed_hash, target, number): (H256, H256, H256, Option<u64>) =
            Deserialize::deserialize(deserializer)?;
        Ok(Work {
            pow_hash,
            seed_hash,
            target,
            number,
        })
    }
}

impl Serialize for Work {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self.number.as_ref() {
            Some(num) => (&self.pow_hash, &self.seed_hash, &self.target, U256::from(*num)).serialize(s),
            None => (&self.pow_hash, &self.seed_hash, &self.target).serialize(s),
        }
    }
}
