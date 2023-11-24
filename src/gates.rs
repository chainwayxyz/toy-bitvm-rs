use crate::{wire::Wire, traits::gate::GateTrait};

// Every gate has a type parameter COM, which is a bit commitment scheme which can be hash based or schnorr based.
// Every gate has an array of input wire pointers.
pub struct NotGate {
    pub input_wires: Vec<*mut Wire>,
    pub output_wires: Vec<*mut Wire>,
}

impl GateTrait for NotGate {
    fn new(input_wires: Vec<*mut Wire>, output_wires: Vec<*mut Wire>) -> Self {
        Self {
            input_wires,
            output_wires,
        }
    }
}