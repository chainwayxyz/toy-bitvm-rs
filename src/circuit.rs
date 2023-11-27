use std::collections::HashMap;

use crate::{traits::{gate::GateTrait, circuit::CircuitTrait}, gates::{NotGate, AndGate, XorGate}, wire::Wire};
use crate::utils::read_lines;

pub struct Circuit {
    pub input_sizes: Vec<usize>,
    pub output_sizes: Vec<usize>,
    pub gates: Vec<Box<dyn GateTrait>>,
    pub wires: Vec<Wire>
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

impl CircuitTrait for Circuit {
    fn evaluate(&self) {

    }

    fn from_bristol(file: &str) -> Self {
        let mut nog: usize = 0; // number of gates
        let mut now: usize = 0; // number of wires
        let mut input_sizes = Vec::<usize>::new();
        let mut output_sizes = Vec::<usize>::new();
        let mut gates = Vec::<Box<dyn GateTrait>>::new();
        let mut wire_indices = HashMap::new();

        for (i, line) in read_lines(file).unwrap().enumerate() {
            if let Ok(line_str) = line {
                if i == 0 {
                    let mut words = line_str.split_whitespace();
                    nog = words.next().unwrap().parse().unwrap();
                    now = words.next().unwrap().parse().unwrap();
                }
                else if i == 1 {
                    let mut words = line_str.split_whitespace();
                    for _ in 0..words.next().unwrap().parse().unwrap() {
                        let x: usize = words.next().unwrap().parse().unwrap();
                        input_sizes.push(x);
                    }
                }
                else if i == 2 {
                    let mut words = line_str.split_whitespace();
                    for _ in 0..words.next().unwrap().parse().unwrap() {
                        let x: usize = words.next().unwrap().parse().unwrap();
                        output_sizes.push(x);
                    }
                }
                else if line_str != "" {
                    let mut words = line_str.split_whitespace();
                    let noi = words.next().unwrap().parse().unwrap(); // number of inputs
                    let noo = words.next().unwrap().parse().unwrap(); // number of outputs
                    let input_wires = (0..noi).map(|_| wire_indices.entry(words.next().unwrap().parse::<usize>().unwrap()).or_insert(Wire::new()).to_owned()).collect();
                    let output_wires = (0..noo).map(|_| wire_indices.entry(words.next().unwrap().parse::<usize>().unwrap()).or_insert(Wire::new()).to_owned()).collect();
                    let gate_type = words.next().unwrap();
                    
                    if gate_type.to_lowercase() == "not" {
                        let gate = NotGate {
                            input_wires,
                            output_wires,
                        };
                        gates.push(Box::new(gate));
                    }
                    else if gate_type.to_lowercase() == "and" {
                        let gate = AndGate {
                            input_wires,
                            output_wires,
                        };
                        gates.push(Box::new(gate));
                    }
                    else if gate_type.to_lowercase() == "xor" {
                        let gate = XorGate {
                            input_wires,
                            output_wires,
                        };
                        gates.push(Box::new(gate));
                    }
                    else {
                        panic!("unknown gate type");
                    }
                }
            }
        }

        assert_eq!(nog, gates.len());
        assert_eq!(wire_indices.keys().min().unwrap().to_owned(), 0);
        assert_eq!(wire_indices.keys().max().unwrap().to_owned(), now - 1);

        return Circuit {
            input_sizes,
            output_sizes,
            gates,
            wires: wire_indices.values().cloned().collect::<Vec<Wire>>(),
        }
    }

    fn generate_commitment_tree(&self) {

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

    #[test]
    fn test_bristol() {
        let circuit = Circuit::from_bristol("bristol/add.txt");
        assert!(circuit.output_sizes[0] == 64);
    }
}

