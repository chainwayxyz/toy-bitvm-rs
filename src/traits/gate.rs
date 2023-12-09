use bitcoin::ScriptBuf;

use crate::wire::HashValue;

pub trait GateTrait {
    fn evaluate(&mut self);
    fn create_response_script(&self, lock_hash: HashValue) -> ScriptBuf;
    fn get_input_size(&self) -> usize;
    fn get_output_size(&self) -> usize;
    fn run_gate_on_inputs(&self, inputs: Vec<bool>) -> Vec<bool>;
}
