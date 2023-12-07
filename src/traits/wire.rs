use bitcoin::{ScriptBuf, XOnlyPublicKey};

pub trait WireTrait {
    fn generate_anti_contradiction_script(&self, verifier_pk: XOnlyPublicKey) -> ScriptBuf;
    fn generate_bit_commitment_script(&self, prover_pk: XOnlyPublicKey) -> ScriptBuf;
}
