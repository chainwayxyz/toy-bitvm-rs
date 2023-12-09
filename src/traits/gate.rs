use crate::wire::{HashValue, Wire};
use bitcoin::ScriptBuf;
use std::{
    iter::zip,
    sync::{Arc, Mutex},
};

pub type Wires = Vec<Arc<Mutex<Wire>>>;

pub trait GateTrait {
    fn get_input_size(&self) -> usize;
    fn get_output_size(&self) -> usize;
    fn get_input_wires(&mut self) -> &mut Wires;
    fn get_output_wires(&mut self) -> &mut Wires;
    fn get_input_bits(&mut self) -> Vec<bool> {
        self.get_input_wires()
            .iter()
            .map(|wire_arcm| wire_arcm.lock().unwrap().selector.unwrap())
            .collect()
    }
    fn set_output_bits(&mut self, output_bits: Vec<bool>) {
        for (wire_arcm, b) in zip(&mut self.get_output_wires().iter(), output_bits) {
            wire_arcm.lock().unwrap().selector = Some(b);
        }
    }
    fn evaluate(&mut self) {
        let input_bits = self.get_input_bits();
        let output_bits = self.run_gate_on_inputs(input_bits);
        self.set_output_bits(output_bits);
    }
    fn create_response_script(&self, lock_hash: HashValue) -> ScriptBuf;
    fn run_gate_on_inputs(&self, inputs: Vec<bool>) -> Vec<bool>;
}
