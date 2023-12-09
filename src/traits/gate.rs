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

    fn create_response_witness(&mut self, hashlock_preimage: [u8; 32]) -> Vec<[u8; 32]> {
        let input_preimages = self
            .get_input_wires()
            .iter()
            .map(|wire_arcm| wire_arcm.lock().unwrap().get_preimage_of_selector())
            .collect::<Vec<[u8; 32]>>();
        let output_preimages = self
            .get_output_wires()
            .iter()
            .map(|wire_arcm| wire_arcm.lock().unwrap().get_preimage_of_selector())
            .collect::<Vec<[u8; 32]>>();
        let mut witness = input_preimages;
        witness.extend(output_preimages);
        witness.push(hashlock_preimage);
        witness
    }

    fn add_preimages_from_witness(&mut self, witness: Vec<[u8; 32]>) -> Option<Wire> {
        let input_preimages = witness[0..self.get_input_size()].to_vec();
        let output_preimages = witness[self.get_input_size()..].to_vec();
        for (wire_arcm, preimage) in zip(&mut self.get_input_wires().iter(), input_preimages) {
            let found_contradiction = wire_arcm.lock().unwrap().add_preimage(preimage);
            if found_contradiction.is_some() {
                return found_contradiction;
            }
        }
        for (wire_arcm, preimage) in zip(&mut self.get_output_wires().iter(), output_preimages) {
            let found_contradiction = wire_arcm.lock().unwrap().add_preimage(preimage);
            if found_contradiction.is_some() {
                return found_contradiction;
            }
        }
        None
    }

    fn run_gate_on_inputs(&self, inputs: Vec<bool>) -> Vec<bool>;
}
