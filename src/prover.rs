// use bitcoin::Network;
use bitcoin::secp256k1::{rand, Keypair, Secp256k1, SecretKey, XOnlyPublicKey};
// use bitcoin::PublicKey;

pub struct Prover {
    pub secret_key: SecretKey,
    pub public_key: XOnlyPublicKey,
}

impl Default for Prover {
    fn default() -> Self {
        Self::new()
    }
}

impl Prover {
    pub fn new() -> Self {
        let secp = Secp256k1::new();
        let mut rng = rand::thread_rng();
        let elems = secp.generate_keypair(&mut rng);
        let keypair = Keypair::from_secret_key(&secp, &elems.0);
        let xonly = XOnlyPublicKey::from_keypair(&keypair);
        Prover {
            secret_key: keypair.secret_key(),
            public_key: xonly.0,
        }
    }

    pub fn get_public_key(&self) -> XOnlyPublicKey {
        self.public_key
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

    #[test]
    fn test_prover_get_public_key() {
        let prover = Prover::new();
        let public_key = prover.get_public_key();
        println!("public key: {:?}", public_key);
    }
}
