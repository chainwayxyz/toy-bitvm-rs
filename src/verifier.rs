// use bitcoin::Network;
use bitcoin::secp256k1::{rand, Keypair, Secp256k1, SecretKey, XOnlyPublicKey};
// use bitcoin::PublicKey;

pub struct Verifier {
    pub secret_key: SecretKey,
    pub public_key: XOnlyPublicKey,
}

impl Default for Verifier {
    fn default() -> Self {
        Self::new()
    }
}

impl Verifier {
    pub fn new() -> Self {
        let secp = Secp256k1::new();
        let mut rng = rand::thread_rng();
        let elems = secp.generate_keypair(&mut rng);
        let keypair = Keypair::from_secret_key(&secp, &elems.0);
        let xonly = XOnlyPublicKey::from_keypair(&keypair);
        Verifier {
            secret_key: keypair.secret_key(),
            public_key: xonly.0,
        }
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
}