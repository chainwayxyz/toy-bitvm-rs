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
    pub zero: Option<PreimageValue>,
    pub one: Option<PreimageValue>,
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
                zero: Some(preimage1),
                one: Some(preimage2),
            }),
            hashes: HashTuple {
                zero: hash1,
                one: hash2,
            },
            selector: None,
            index: Some(index),
        }
    }

    pub fn new_with_hash_pair(index: usize, hash_pair: HashTuple) -> Self {
        Wire {
            preimages: None,
            hashes: hash_pair,
            selector: None,
            index: Some(index),
        }
    }

    pub fn get_hash_pair(&self) -> HashTuple {
        self.hashes
    }

    pub fn get_preimage_of_selector(&self) -> [u8; 32] {
        match self.preimages {
            Some(preimage_tuple) => match self.selector {
                Some(b) => {
                    if !b {
                        preimage_tuple.zero.unwrap()
                    } else {
                        preimage_tuple.one.unwrap()
                    }
                }
                None => panic!("selector is not set"),
            },
            None => panic!("preimages are not set"),
        }
    }

    pub fn add_preimage(&mut self, preimage: PreimageValue) -> Option<Wire> {
        let hash = sha256::Hash::hash(&preimage).to_byte_array();
        if hash == self.hashes.zero {
            self.preimages = Some(PreimageTuple {
                zero: Some(preimage),
                one: match self.preimages {
                    Some(cur) => cur.one,
                    None => None,
                },
            });
        } else if hash == self.hashes.one {
            self.preimages = Some(PreimageTuple {
                zero: match self.preimages {
                    Some(cur) => cur.zero,
                    None => None,
                },
                one: Some(preimage),
            });
        } else {
            panic!("preimage does not match either hash");
        }
        if self.preimages.unwrap().zero.is_some() && self.preimages.unwrap().one.is_some() {
            return Some(self.clone());
        }
        None
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
