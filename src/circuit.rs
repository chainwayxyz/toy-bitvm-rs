use crate::{traits::{gate::GateTrait, wire::WireTrait}, gates::NotGate};

pub struct Circuit {
    pub input_sizes: Vec<usize>,
    pub output_sizes: Vec<usize>,
    pub gates: Vec<Box<dyn GateTrait>>,
    pub wires: Vec<Box<dyn WireTrait>>
}

impl Circuit {
    pub fn new() -> Self {
        return Circuit {
            input_sizes: vec![32, 32],
            output_sizes: vec![32],
            gates: vec![Box::new(NotGate::new(vec![], vec![]))],
            wires: vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circuit() {
        let circuit = Circuit::new();
        assert!(circuit.output_sizes[0] == 32);
    }
}

