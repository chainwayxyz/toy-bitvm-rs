use bitcoin::{ScriptBuf, XOnlyPublicKey, script::Builder};

pub trait WireTrait {
    fn generate_anti_contradiction_script(&self, verifier_pk: XOnlyPublicKey) -> ScriptBuf;
    fn add_bit_commitment_script(&self, builder: Builder) -> Builder;
}
