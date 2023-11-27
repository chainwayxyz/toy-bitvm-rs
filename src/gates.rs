use crate::{wire::Wire, traits::gate::GateTrait};

// Every gate has a type parameter COM, which is a bit commitment scheme which can be hash based or schnorr based.
// Every gate has an array of input wire pointers.
pub struct NotGate {
    pub input_wires: Vec<*mut Wire>,
    pub output_wires: Vec<*mut Wire>,
}

impl NotGate {
    pub fn new(input_wires: Vec<*mut Wire>, output_wires: Vec<*mut Wire>) -> Self {
        return NotGate {
            input_wires,
            output_wires,
        }
    }
}

impl GateTrait for NotGate {
    fn create_challenge_script(&self) -> String {
        return "NotGate".to_string();
    }
}