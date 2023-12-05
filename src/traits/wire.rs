use bitcoin::{ScriptBuf, XOnlyPublicKey};

pub trait WireTrait {
    fn generate_anti_contradiction_script(&self, verifier_pk: XOnlyPublicKey) -> ScriptBuf;
}
