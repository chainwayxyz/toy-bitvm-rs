// use bitcoin::Network;
use bitcoin::{
    hashes::Hash,
    secp256k1::{
        rand, schnorr::Signature, All, Keypair, Message, Secp256k1, SecretKey, XOnlyPublicKey,
    },
    Address, TapSighash, TapTweakHash,
};
// use bitcoin::PublicKey;

pub struct Prover {
    secp: Secp256k1<All>,
    keypair: Keypair,
    pub secret_key: SecretKey,
    pub public_key: XOnlyPublicKey,
    pub address: Address,
}

impl Default for Prover {
    fn default() -> Self {
        Self::new()
    }
}

impl Prover {
    pub fn new() -> Self {
        let secp: Secp256k1<All> = Secp256k1::new();
        let mut rng = rand::thread_rng();
        let elems = secp.generate_keypair(&mut rng);
        let keypair = Keypair::from_secret_key(&secp, &elems.0);
        let xonly = XOnlyPublicKey::from_keypair(&keypair);
        let address = Address::p2tr(&secp, xonly.0, None, bitcoin::Network::Signet);
        
        Prover {
            secp,
            keypair,
            secret_key: keypair.secret_key(),
            public_key: xonly.0,
            address,
        }
    }

    pub fn sign(&self, sighash: TapSighash) -> Signature {
        self.secp.sign_schnorr_with_rng(
            &Message::from_digest_slice(sighash.as_byte_array()).expect("should be hash"),
        &self.keypair.add_xonly_tweak(&self.secp, &TapTweakHash::from_key_and_tweak(self.public_key, None).to_scalar()).unwrap(),
            &mut rand::thread_rng(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prover() {
        let prover = Prover::new();
        println!("secret key: {:?}", prover.secret_key);
        println!("public key: {:?}", prover.public_key);
    }
}
