
use std::collections::BTreeMap;
use std::iter::zip;

use std::sync::{Mutex, Arc};

use crate::wire::HashTuple;

use crate::{
    gates::{AndGate, NotGate, XorGate},
    traits::gate::GateTrait,
    utils::read_lines,
    wire::Wire,
};

pub struct Circuit {
    pub input_sizes: Vec<usize>,
    pub output_sizes: Vec<usize>,
    pub gates: Vec<Box<dyn GateTrait + Send>>,
    pub wires: Vec<Arc<Mutex<Wire>>>,
}

impl Default for Circuit {
    fn default() -> Self {
        Self::from_bristol("bristol/test.txt", None)
    }
}

impl Circuit {
    pub fn num_gates(&self) -> usize {
        self.gates.len()
    }

    pub fn evaluate(&mut self, inputs: Vec<Vec<bool>>) -> Vec<Vec<bool>> {
        assert_eq!(
            inputs.len(),
            self.input_sizes.len(),
            "wrong number of inputs"
        );
        let mut combined_inputs = Vec::new();
        for (a, b) in zip(inputs, self.input_sizes.clone()) {
            assert_eq!(
                a.len(),
                b,
                "input lengths do not match for one of the inputs"
            );
            combined_inputs.extend(a);
        }
        for (i, value) in combined_inputs.iter().enumerate() {
            self.wires[i].lock().unwrap().selector = Some(*value);
        }
        for gate in self.gates.as_mut_slice() {
            gate.evaluate();
        }
        let mut output = Vec::new();
        let total_output_size = self.output_sizes.iter().sum::<usize>();
        let mut output_index = self.wires.len() - total_output_size;
        for os in self.output_sizes.clone() {
            let mut output_vec = Vec::new();
            for i in output_index..(output_index + os) {
                let value = self.wires[i].lock().unwrap().selector.unwrap();
                output_vec.push(value);
            }
            output_index += os;
            output.push(output_vec);
        }
        output
    }

    pub fn get_wire_hashes(&self) -> Vec<HashTuple> {
        self.wires
            .iter()
            .map(|wire_rcref| {
                let wire = wire_rcref.lock().unwrap();
                wire.get_hash_pair()
            })
            .collect::<Vec<HashTuple>>()
    }

    pub fn from_bristol(file: &str, wire_hashes: Option<Vec<HashTuple>>) -> Self {
        let mut nog: usize = 0; // number of gates
        let mut now: usize = 0; // number of wires
        let mut input_sizes = Vec::<usize>::new();
        let mut output_sizes = Vec::<usize>::new();
        let mut gates = Vec::<Box<dyn GateTrait + Send>>::new();
        let mut wire_indices = BTreeMap::new();

        for (i, line) in read_lines(file).unwrap().enumerate() {
            if let Ok(line_str) = line {
                if i == 0 {
                    let mut words = line_str.split_whitespace();
                    nog = words.next().unwrap().parse().unwrap();
                    now = words.next().unwrap().parse().unwrap();
                    for i in 0..now {
                        let wire = if let Some(wire_hashes) = wire_hashes.clone() {
                            Wire::new_with_hash_pair(i, wire_hashes[i])
                        } else {
                            Wire::new(i)
                        };
                        wire_indices.insert(i, Arc::new(Mutex::new(wire)));
                    }
                } else if i == 1 {
                    let mut words = line_str.split_whitespace();
                    for _ in 0..words.next().unwrap().parse().unwrap() {
                        let x: usize = words.next().unwrap().parse().unwrap();
                        input_sizes.push(x);
                    }
                } else if i == 2 {
                    let mut words = line_str.split_whitespace();
                    for _ in 0..words.next().unwrap().parse().unwrap() {
                        let x: usize = words.next().unwrap().parse().unwrap();
                        output_sizes.push(x);
                    }
                } else if !line_str.is_empty() {
                    let mut words = line_str.split_whitespace();
                    let noi = words.next().unwrap().parse().unwrap(); // number of inputs
                    let noo = words.next().unwrap().parse().unwrap(); // number of outputs
                    let input_wires = (0..noi)
                        .map(|_| {
                            let k = words.next().unwrap().parse::<usize>().unwrap();
                            let x = wire_indices.get(&k).unwrap().clone();
                            x
                        })
                        .collect();
                    let output_wires = (0..noo)
                        .map(|_| {
                            let k = words.next().unwrap().parse::<usize>().unwrap();
                            let x = wire_indices.get(&k).unwrap().clone();
                            x
                        })
                        .collect();
                    let gate_type = words.next().unwrap();

                    if ["not".to_string(), "inv".to_string()].contains(&gate_type.to_lowercase()) {
                        let gate = NotGate {
                            input_wires,
                            output_wires,
                        };
                        gates.push(Box::new(gate));
                    } else if ["and".to_string()].contains(&gate_type.to_lowercase()) {
                        let gate = AndGate {
                            input_wires,
                            output_wires,
                        };
                        gates.push(Box::new(gate));
                    } else if ["xor".to_string()].contains(&gate_type.to_lowercase()) {
                        let gate = XorGate {
                            input_wires,
                            output_wires,
                        };
                        gates.push(Box::new(gate));
                    } else {
                        panic!("unknown gate type");
                    }
                }
            }
        }

        assert_eq!(nog, gates.len(), "wrong number of gates");
        assert_eq!(
            wire_indices.keys().min().unwrap().to_owned(),
            0,
            "wires should start 0"
        );
        assert_eq!(
            wire_indices.keys().max().unwrap().to_owned(),
            now - 1,
            "wires should end at the specified number"
        );
        assert!(
            input_sizes.iter().sum::<usize>() + output_sizes.iter().sum::<usize>() <= now,
            "not enough wires for inputs and outputs"
        );

        return Circuit {
            input_sizes,
            output_sizes,
            gates,
            wires: wire_indices
                .values()
                .cloned()
                .collect::<Vec<Arc<Mutex<Wire>>>>(),
        };
    }
}

#[cfg(test)]
mod tests {
    use bitcoin::secp256k1::Secp256k1;
    use bitcoin::taproot::LeafVersion;

    use super::*;
    use crate::actor::Actor;
    use crate::utils::{bool_array_to_number, number_to_bool_array};

    use crate::transactions::{
        generate_anti_contradiction_script, generate_challenge_address_and_info,
        generate_timelock_script,
    };

    #[test]
    fn test_circuit() {
        let circuit = Circuit::default();
        assert!(circuit.output_sizes[0] == 1);
    }

    #[test]
    fn test_bristol() {
        let circuit = Circuit::from_bristol("bristol/add.txt", None);
        assert!(circuit.output_sizes[0] == 64);
    }

    #[test]
    fn test_add_circuit() {
        let mut circuit = Circuit::from_bristol("bristol/add.txt", None);
        let a1 = 633;
        let a2 = 15;
        let b1 = number_to_bool_array(a1, 64);
        let b2 = number_to_bool_array(a2, 64);

        let o = circuit.evaluate(vec![b1, b2]);
        let output = bool_array_to_number(o.get(0).unwrap().to_vec());
        assert_eq!(output, a1 + a2);
    }

    #[test]
    fn test_challenge_tree() {
        let circuit = Circuit::from_bristol("bristol/test.txt", None);
        let prover = Actor::new();
        let mut verifier = Actor::new();
        let secp = Secp256k1::new();

        let challenge_hashes = verifier.generate_challenge_hashes(circuit.num_gates());

        let (_address, tree_info) = generate_challenge_address_and_info(
            &secp,
            &circuit,
            prover.public_key,
            verifier.public_key,
            &challenge_hashes,
        );
        for wire_rcref in circuit.wires.iter() {
            let wire = wire_rcref.lock().unwrap();
            let script =
                generate_anti_contradiction_script(wire.get_hash_pair(), verifier.public_key);
            let ctrl_block = tree_info
                .control_block(&(script.clone(), LeafVersion::TapScript))
                .unwrap();
            assert!(ctrl_block.verify_taproot_commitment(
                &secp,
                tree_info.output_key().to_inner(),
                &script
            ));
        }
        // TODO: add tests for reveral challenge scripts
        let p10_script = generate_timelock_script(prover.public_key, 10);
        let p10_ctrl_block = tree_info
            .control_block(&(p10_script.clone(), LeafVersion::TapScript))
            .unwrap();
        assert!(p10_ctrl_block.verify_taproot_commitment(
            &secp,
            tree_info.output_key().to_inner(),
            &p10_script
        ));
    }
}
