use std::fmt::Debug;

use bitcoin::hashes::sha256;
use bitcoin::hashes::Hash;
use rand::Rng;
use serde::Deserialize;
use serde::Serialize;

pub type HashValue = [u8; 32];
pub type PreimageValue = [u8; 32];

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct HashTuple {
    pub zero: HashValue,
    pub one: HashValue,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct PreimageTuple {
    pub zero: PreimageValue,
    pub one: PreimageValue,
}

#[derive(Clone)]
pub struct Wire {
    pub preimages: Option<PreimageTuple>,
    pub hashes: HashTuple,
    pub selector: Option<bool>,
    pub index: Option<usize>,
}

impl Debug for Wire {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Wire[{:?}]: {:?}", self.index, self.selector)
    }
}

impl Default for Wire {
    fn default() -> Self {
        Self::new(0)
    }
}

impl Wire {
    pub fn new(index: usize) -> Self {
        let mut rng = rand::thread_rng();

        let preimage1: [u8; 32] = rng.gen();
        let preimage2: [u8; 32] = rng.gen();

        let hash1 = sha256::Hash::hash(&preimage1).to_byte_array();
        let hash2 = sha256::Hash::hash(&preimage2).to_byte_array();

        Wire {
            preimages: Some(PreimageTuple {
                zero: preimage1,
                one: preimage2,
            }),
            hashes: HashTuple {
                zero: hash1,
                one: hash2,
            },
            selector: None,
            index: Some(index),
        }
    }

    pub fn get_hash_pair(&self) -> HashTuple {
        self.hashes
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_wire() {
        let wire = Wire::new(0);
        assert!(wire.preimages.is_some());
        assert!(wire.selector.is_none());
    }
}
