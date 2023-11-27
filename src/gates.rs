use crate::{traits::gate::GateTrait, wire::Wire};

// Every gate has a type parameter COM, which is a bit commitment scheme which can be hash based or schnorr based.
// Every gate has an array of input wire pointers.
pub struct NotGate {
    pub input_wires: Vec<Wire>,
    pub output_wires: Vec<Wire>,
}

impl NotGate {
    pub fn new(input_wires: Vec<Wire>, output_wires: Vec<Wire>) -> Self {
        return NotGate {
            input_wires,
            output_wires,
        };
    }
}

impl GateTrait for NotGate {
    fn create_challenge_script(&self) -> String {
        return "NotGate".to_string();
    }
}

pub struct AndGate {
    pub input_wires: Vec<Wire>,
    pub output_wires: Vec<Wire>,
}

impl AndGate {
    pub fn new(input_wires: Vec<Wire>, output_wires: Vec<Wire>) -> Self {
        return AndGate {
            input_wires,
            output_wires,
        };
    }
}

impl GateTrait for AndGate {
    fn create_challenge_script(&self) -> String {
        return "NotGate".to_string();
    }
}

pub struct XorGate {
    pub input_wires: Vec<Wire>,
    pub output_wires: Vec<Wire>,
}

impl XorGate {
    pub fn new(input_wires: Vec<Wire>, output_wires: Vec<Wire>) -> Self {
        return XorGate {
            input_wires,
            output_wires,
        };
    }
}

impl GateTrait for XorGate {
    fn create_challenge_script(&self) -> String {
        return "NotGate".to_string();
    }
}
