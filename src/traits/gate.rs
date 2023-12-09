//use crate::wire::Wire;
//use std::rc::Rc;
//use std::cell::RefCell;

use bitcoin::ScriptBuf;

pub trait GateTrait {
    fn evaluate(&mut self);
    fn run_gate_on_inputs(&self, inputs: Vec<bool>) -> Vec<bool>;
    fn get_input_size(&self) -> usize;
    fn get_output_size(&self) -> usize;
    fn create_response_script(&self, lock_hash: [u8; 32]) -> ScriptBuf;
}