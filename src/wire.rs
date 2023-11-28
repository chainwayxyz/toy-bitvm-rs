use std::fmt::Debug;

use crate::traits::wire::WireTrait;
use bitcoin::blockdata::script::Builder;
use bitcoin::hashes::sha256;
use bitcoin::hashes::Hash;
use bitcoin::opcodes::all::*;
use bitcoin::ScriptBuf;
use rand::Rng;

#[derive(Clone)]
pub struct Wire {
    pub preimages: Option<[[u8; 32]; 2]>,
    pub hashes: [[u8; 32]; 2],
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
            preimages: Some([preimage1, preimage2]),
            hashes: [hash1, hash2],
            selector: None,
            index: Some(index),
        }
    }
}

impl WireTrait for Wire {
    fn generate_anti_contradiction_script(&self) -> ScriptBuf {
        Builder::new()
            .push_opcode(OP_SHA256)
            .push_slice(self.hashes[0])
            .push_opcode(OP_EQUALVERIFY)
            .push_opcode(OP_SHA256)
            .push_slice(self.hashes[1])
            .push_opcode(OP_EQUAL)
            .into_script()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bitcoin::TapLeafHash;
    use bitcoin::Transaction;
    use bitcoin_scriptexec::*;

    #[test]
    fn test_wire() {
        let wire = Wire::new(0);
        assert!(wire.preimages.is_some());
        assert!(wire.selector.is_none());
    }

    #[test]
    fn test_generate_anti_contradiction_script() {
        let wire = Wire::new(0);
        let script = wire.generate_anti_contradiction_script();

        let preimages_vec = if let Some(preimages) = wire.preimages {
            vec![preimages[1].to_vec(), preimages[0].to_vec()]
        } else {
            panic!("wire preimages are None")
        };

        let mut exec = Exec::new(
            ExecCtx::Tapscript,
            Options::default(),
            TxTemplate {
                tx: Transaction {
                    version: bitcoin::transaction::Version::TWO,
                    lock_time: bitcoin::locktime::absolute::LockTime::ZERO,
                    input: vec![],
                    output: vec![],
                },
                prevouts: vec![],
                input_idx: 0,
                taproot_annex_scriptleaf: Some((TapLeafHash::all_zeros(), None)),
            },
            script,
            preimages_vec,
        )
        .expect("error creating exec");

        loop {
            if exec.exec_next().is_err() {
                break;
            }
        }

        let res = exec.result().unwrap().clone();

        println!("{:?}", res);
        assert_eq!(res.error, None);
    }
}
