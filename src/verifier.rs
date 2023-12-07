use std::str::FromStr;

use bitcoin::blockdata::script::Builder;
use bitcoin::hashes::sha256;
use bitcoin::hashes::Hash;
use bitcoin::opcodes::all::*;
use bitcoin::taproot::LeafVersion;
use bitcoin::{
    secp256k1::{rand, All, Keypair, Secp256k1, SecretKey, XOnlyPublicKey},
    taproot::TaprootBuilder,
};
use bitcoin::{Address, Network};
use rand::Rng;

pub struct Verifier {
    // secp: Secp256k1<All>,
    // keypair: Keypair,
    pub secret_key: SecretKey,
    pub public_key: XOnlyPublicKey,
    pub address: Address
}
impl Default for Verifier {
    fn default() -> Self {
        Self::new()
    }
}

impl Verifier {
    pub fn new() -> Self {
        let secp: Secp256k1<All> = Secp256k1::new();
        let mut rng = rand::thread_rng();
        let elems = secp.generate_keypair(&mut rng);
        let keypair = Keypair::from_secret_key(&secp, &elems.0);
        let xonly = XOnlyPublicKey::from_keypair(&keypair);
        let address = Address::p2tr(&secp, xonly.0, None, bitcoin::Network::Signet);


        Verifier {
            secret_key: keypair.secret_key(),
            public_key: xonly.0,
            address,
        }
    }

    pub fn create_challenge_tree(&self, circuit_size: usize, prover_pk: XOnlyPublicKey) -> (Address, Vec<[u8; 32]>) {
        let m = (circuit_size - 1).ilog2() + 1;
        let mut taproot = TaprootBuilder::new();
        let mut rng = rand::thread_rng();
        let mut hash_vec = Vec::new();
        let k = 2_usize.pow(m) - circuit_size;
        for i in 0..circuit_size {
            let temp: [u8; 32] = rng.gen();
            let temp_hash = sha256::Hash::hash(&temp).to_byte_array();
            hash_vec.push(temp_hash);
            let temp_script = Builder::new()
                .push_opcode(OP_SHA256)
                .push_slice(temp_hash)
                .push_opcode(OP_EQUALVERIFY)
                .into_script();
            println!("temp_script: {:?}", temp_script);

            if i < circuit_size - k {
                taproot = taproot.add_leaf((m + 1) as u8, temp_script).unwrap();
            } else {
                taproot = taproot.add_leaf(m as u8, temp_script).unwrap();
            }
        }
        println!("hash_vec: {:?}", hash_vec);
        println!("taproot: {:?}", taproot);

        let p10_script = Builder::new()
            .push_int(10)
            .push_opcode(OP_CSV)
            .push_x_only_key(&prover_pk)
            .push_opcode(OP_CHECKSIG)
            .into_script();

        taproot = taproot.add_leaf(1, p10_script.clone()).unwrap();
        //TODO: change this to have the verifier's public keys
        let secp = Secp256k1::verification_only();
        let internal_key = XOnlyPublicKey::from_str(
            "93c7378d96518a75448821c4f7c8f4bae7ce60f804d03d1f0628dd5dd0f5de51",
        )
        .unwrap();
        let tree_info = taproot.finalize(&secp, internal_key).unwrap();
        println!("tree_info: {:?}", tree_info);
        let output_key = tree_info.output_key();
        println!("output_key: {:?}", output_key);

        let p10_ver_script = (p10_script, LeafVersion::TapScript);
        let p10_ctrl_block = tree_info.control_block(&p10_ver_script).unwrap();
        assert!(p10_ctrl_block.verify_taproot_commitment(
            &secp,
            output_key.to_inner(),
            &p10_ver_script.0
        ));

        (
            Address::p2tr(
                &secp,
                internal_key,
                tree_info.merkle_root(),
                Network::Signet,
            ),
            hash_vec,
        )
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_verifier() {
        let verifier = Verifier::new();
        println!("secret key: {:?}", verifier.secret_key);
        println!("public key: {:?}", verifier.public_key);
    }

    #[test]
    fn test_create_challenge_tree() {
        let verifier = Verifier::new();
        let (tree, hash_vec) = verifier.create_challenge_tree(3, XOnlyPublicKey::from_str(
            "93c7378d96518a75448821c4f7c8f4bae7ce60f804d03d1f0628dd5dd0f5de51",
        )
        .unwrap());
        println!("tree: {:?}", tree);
        println!("hash_vec: {:?}", hash_vec);
    }
}
