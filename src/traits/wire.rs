use bitcoin::{script::Builder, ScriptBuf, XOnlyPublicKey};

use crate::wire::HashTuple;

pub trait WireTrait {
    fn get_hash_pair(&self) -> HashTuple;
    fn generate_anti_contradiction_script(&self, verifier_pk: XOnlyPublicKey) -> ScriptBuf;
    fn add_bit_commitment_script(&self, builder: Builder) -> Builder;
}
