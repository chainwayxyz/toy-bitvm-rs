use bitcoin::ScriptBuf;

pub trait WireTrait {
    fn new() -> Self;
    fn generate_anti_contradiction_script(&self) -> ScriptBuf;
}
