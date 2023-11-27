use crate::traits::wire::WireTrait;
use bitcoin::blockdata::script::Builder;
use bitcoin::blockdata::script::Instruction;
use bitcoin::hashes::sha256;
use bitcoin::hashes::Hash;
use bitcoin::opcodes::all::*;
use bitcoin::Script;
use bitcoin::ScriptBuf;
use bitcoin::Target;
use bitcoin::opcodes::all::*;
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
    fn generate_anti_contradiction_script(&self) -> ScriptBuf {
        Builder::new()
            .push_opcode(OP_SHA256)
            .push_slice(&self.hashes[0].to_le_bytes())
            .push_opcode(OP_EQUALVERIFY)
            .push_opcode(OP_SHA256)
            .push_slice(&self.hashes[1].to_le_bytes())
            .push_opcode(OP_EQUAL)
            .into_script()
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

    #[test]
    fn test_generate_anti_contradiction_script() {
        let wire = Wire::new();
        let _script = wire.generate_anti_contradiction_script();
        // TODO:Test if script returns 1 given input witness with [preimages[0], preimages[1]
    }
}

