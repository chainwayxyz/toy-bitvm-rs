use bitcoin::{script::Builder, ScriptBuf, XOnlyPublicKey};

pub trait WireTrait {
    fn get_hash_pair(&self) -> [[u8; 32]; 2];
    fn generate_anti_contradiction_script(&self, verifier_pk: XOnlyPublicKey) -> ScriptBuf;
    fn add_bit_commitment_script(&self, builder: Builder) -> Builder;
}
