use crate::traits::wire::WireTrait;
use bitcoin::Script;
use bitcoin::hashes::sha256;
use bitcoin::hashes::Hash;
use bitcoin::Target;
use rand::Rng;

pub struct Wire {
    pub preimages: Option<[Target; 2]>,
    pub hashes: [Target; 2],
    pub selector: Option<bool>,
}

impl Wire {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();

        let preimage1 = Target::from_le_bytes(rng.gen());
        let preimage2 = Target::from_le_bytes(rng.gen());

        let hash1 =
            Target::from_le_bytes(sha256::Hash::hash(&preimage1.to_le_bytes()).to_byte_array());
        let hash2 =
            Target::from_le_bytes(sha256::Hash::hash(&preimage2.to_le_bytes()).to_byte_array());

        return Wire {
            preimages: Some([preimage1, preimage2]),
            hashes: [hash1, hash2],
            selector: None,
        };
    }
}

impl WireTrait for Wire {
    fn create_commit_script(&self) -> Box<&Script> {
        return Box::new(Script::new());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wire() {
        let wire = Wire::new();
        assert_eq!(wire.preimages.is_some(), true);
        assert_eq!(wire.selector.is_none(), true);
    }
}
