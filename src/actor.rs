use bitcoin::hashes::sha256;
use bitcoin::TapNodeHash;
use bitcoin::{
    hashes::Hash,
    secp256k1::{
        rand, schnorr::Signature, All, Keypair, Message, Secp256k1, SecretKey, XOnlyPublicKey,
    },
    Address, TapSighash, TapTweakHash,
};
use rand::Rng;

use crate::circuit::wire::{HashValue, PreimageValue};

pub struct Actor {
    secp: Secp256k1<All>,
    keypair: Keypair,
    pub secret_key: SecretKey,
    pub public_key: XOnlyPublicKey,
    pub address: Address,
    challenge_preimages: Vec<Vec<PreimageValue>>,
    challenge_hashes: Vec<Vec<HashValue>>,
    signatures: Vec<Signature>,
}

impl Default for Actor {
    fn default() -> Self {
        Self::new()
    }
}

impl Actor {
    pub fn new() -> Self {
        let secp: Secp256k1<All> = Secp256k1::new();
        let mut rng = rand::thread_rng();
        let (sk, _pk) = secp.generate_keypair(&mut rng);
        let keypair = Keypair::from_secret_key(&secp, &sk);
        let (xonly, _parity) = XOnlyPublicKey::from_keypair(&keypair);
        let address = Address::p2tr(&secp, xonly, None, bitcoin::Network::Regtest);

        Actor {
            secp,
            keypair,
            secret_key: keypair.secret_key(),
            public_key: xonly,
            address,
            challenge_preimages: Vec::new(),
            challenge_hashes: Vec::new(),
            signatures: Vec::new(),
        }
    }

    pub fn sign_with_tweak(
        &self,
        sighash: TapSighash,
        merkle_root: Option<TapNodeHash>,
    ) -> Signature {
        self.secp.sign_schnorr_with_rng(
            &Message::from_digest_slice(sighash.as_byte_array()).expect("should be hash"),
            &self
                .keypair
                .add_xonly_tweak(
                    &self.secp,
                    &TapTweakHash::from_key_and_tweak(self.public_key, merkle_root).to_scalar(),
                )
                .unwrap(),
            &mut rand::thread_rng(),
        )
    }

    pub fn sign(&self, sighash: TapSighash) -> Signature {
        self.secp.sign_schnorr_with_rng(
            &Message::from_digest_slice(sighash.as_byte_array()).expect("should be hash"),
            &self.keypair,
            &mut rand::thread_rng(),
        )
    }

    pub fn generate_challenge_hashes(&mut self, num_gates: usize) -> Vec<HashValue> {
        let mut challenge_hashes: Vec<HashValue> = Vec::new();
        let mut rng = rand::thread_rng();
        let mut preimages = Vec::new();
        for _ in 0..num_gates {
            let preimage: PreimageValue = rng.gen();
            preimages.push(preimage);
            challenge_hashes.push(sha256::Hash::hash(&preimage).to_byte_array());
        }
        self.challenge_preimages.push(preimages);
        self.challenge_hashes.push(challenge_hashes.clone());
        challenge_hashes
    }

    pub fn add_challenge_hashes(&mut self, challenge_hashes: Vec<HashValue>) {
        self.challenge_hashes.push(challenge_hashes);
    }

    pub fn get_challenge_hashes(&self, index: usize) -> Vec<HashValue> {
        self.challenge_hashes[index].clone()
    }

    pub fn get_challenge_preimage(&self, index: usize, gate_num: usize) -> PreimageValue {
        self.challenge_preimages[index][gate_num]
    }

    pub fn add_signature(&mut self, signature: Signature) {
        self.signatures.push(signature);
    }

    pub fn get_signature(&self, index: usize) -> Signature {
        self.signatures[index]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prover() {
        let prover = Actor::new();
        println!("secret key: {:?}", prover.secret_key);
        println!("public key: {:?}", prover.public_key);
    }
}
