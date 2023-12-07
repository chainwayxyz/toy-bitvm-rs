use bitcoin::opcodes::all::*;
use bitcoin::{
    hashes::Hash,
    script::Builder,
    secp256k1::{
        rand, schnorr::Signature, All, Keypair, Message, Secp256k1, SecretKey, XOnlyPublicKey,
    },
    Address, ScriptBuf, TapSighash, TapTweakHash,
};

pub struct Actor {
    secp: Secp256k1<All>,
    keypair: Keypair,
    pub secret_key: SecretKey,
    pub public_key: XOnlyPublicKey,
    pub address: Address,
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
        let address = Address::p2tr(&secp, xonly, None, bitcoin::Network::Signet);

        Actor {
            secp,
            keypair,
            secret_key: keypair.secret_key(),
            public_key: xonly,
            address,
        }
    }

    pub fn generate_timelock_script(&self, block_count: u32) -> ScriptBuf {
        Builder::new()
            .push_int(block_count as i64)
            .push_opcode(OP_CSV)
            .push_x_only_key(&self.public_key)
            .push_opcode(OP_CHECKSIG)
            .into_script()
    }

    pub fn sign(&self, sighash: TapSighash) -> Signature {
        self.secp.sign_schnorr_with_rng(
            &Message::from_digest_slice(sighash.as_byte_array()).expect("should be hash"),
            &self
                .keypair
                .add_xonly_tweak(
                    &self.secp,
                    &TapTweakHash::from_key_and_tweak(self.public_key, None).to_scalar(),
                )
                .unwrap(),
            &mut rand::thread_rng(),
        )
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
