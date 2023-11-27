use bitcoin::ScriptBuf;

pub trait WireTrait {
    fn generate_anti_contradiction_script(&self) -> ScriptBuf;
}
