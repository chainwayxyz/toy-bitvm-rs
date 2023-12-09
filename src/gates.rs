use bitcoin::opcodes::all::{
    OP_EQUALVERIFY, OP_FROMALTSTACK, OP_NOT, OP_SHA256, OP_TOALTSTACK, OP_BOOLAND, OP_NUMEQUAL,
};
use bitcoin::script::Builder;
use bitcoin::ScriptBuf;

use crate::traits::wire::WireTrait;
use crate::{traits::gate::GateTrait, wire::Wire};
use std::cell::RefCell;
use std::rc::Rc;

// Every gate has a type parameter COM, which is a bit commitment scheme which can be hash based or schnorr based.
// Every gate has an array of input wire pointers.
#[derive(Clone)]
pub struct NotGate {
    pub input_wires: Vec<Rc<RefCell<Wire>>>,
    pub output_wires: Vec<Rc<RefCell<Wire>>>,
}

impl NotGate {
    pub fn new(input_wires: Vec<Rc<RefCell<Wire>>>, output_wires: Vec<Rc<RefCell<Wire>>>) -> Self {
        NotGate {
            input_wires,
            output_wires,
        }
    }
}

impl GateTrait for NotGate {
    fn evaluate(&mut self) {
        let in1 = &mut self.input_wires[0].try_borrow_mut().unwrap();
        let out = &mut self.output_wires[0].try_borrow_mut().unwrap();
        let u = in1.selector.as_mut().unwrap();
        let w = !*u;
        out.selector = Some(w);
    }

    fn run_gate_on_inputs(&self, inputs: Vec<bool>) -> Vec<bool> {
        assert!(inputs.len() == 1);
        vec![!inputs[0]]
    }

    fn get_input_size(&self) -> usize {
        1
    }

    fn get_output_size(&self) -> usize {
        1
    }

    fn create_response_script(&self, lock_hash: [u8; 32]) -> ScriptBuf {
        let builder = Builder::new()
            .push_opcode(OP_SHA256)
            .push_slice(lock_hash)
            .push_opcode(OP_EQUALVERIFY);
        let builder = self.output_wires[0]
            .try_borrow()
            .unwrap()
            .add_bit_commitment_script(builder)
            .push_opcode(OP_TOALTSTACK);
        let builder = self.input_wires[0]
            .try_borrow()
            .unwrap()
            .add_bit_commitment_script(builder);
        builder
            .push_opcode(OP_NOT)
            .push_opcode(OP_FROMALTSTACK)
            .push_opcode(OP_EQUALVERIFY)
            .into_script()
    }
}

#[derive(Clone)]
pub struct AndGate {
    pub input_wires: Vec<Rc<RefCell<Wire>>>,
    pub output_wires: Vec<Rc<RefCell<Wire>>>,
}

impl AndGate {
    pub fn new(input_wires: Vec<Rc<RefCell<Wire>>>, output_wires: Vec<Rc<RefCell<Wire>>>) -> Self {
        AndGate {
            input_wires,
            output_wires,
        }
    }
}

impl GateTrait for AndGate {
    fn evaluate(&mut self) {
        let in1 = &mut self.input_wires[0].try_borrow_mut().unwrap();
        let in2 = &mut self.input_wires[1].try_borrow_mut().unwrap();
        let out = &mut self.output_wires[0].try_borrow_mut().unwrap();
        let u = in1.selector.as_mut().unwrap();
        let v = in2.selector.as_mut().unwrap();
        let w = *u && *v;
        out.selector = Some(w);
    }

    fn run_gate_on_inputs(&self, inputs: Vec<bool>) -> Vec<bool> {
        assert!(inputs.len() == 2);
        vec![inputs[0] && inputs[1]]
    }

    fn get_input_size(&self) -> usize {
        2
    }

    fn get_output_size(&self) -> usize {
        1
    }

    fn create_response_script(&self, lock_hash: [u8; 32]) -> ScriptBuf {
        let builder = Builder::new()
            .push_opcode(OP_SHA256)
            .push_slice(lock_hash)
            .push_opcode(OP_EQUALVERIFY);
        let builder = self.output_wires[0]
            .try_borrow()
            .unwrap()
            .add_bit_commitment_script(builder)
            .push_opcode(OP_TOALTSTACK);
        let builder = self.input_wires[1]
            .try_borrow()
            .unwrap()
            .add_bit_commitment_script(builder)
            .push_opcode(OP_TOALTSTACK);
        let builder = self.input_wires[0]
            .try_borrow()
            .unwrap()
            .add_bit_commitment_script(builder);
        builder
            .push_opcode(OP_FROMALTSTACK)
            .push_opcode(OP_BOOLAND)
            .push_opcode(OP_FROMALTSTACK)
            .push_opcode(OP_EQUALVERIFY)
            .into_script()
    }
}

#[derive(Clone)]
pub struct XorGate {
    pub input_wires: Vec<Rc<RefCell<Wire>>>,
    pub output_wires: Vec<Rc<RefCell<Wire>>>,
}

impl XorGate {
    pub fn new(input_wires: Vec<Rc<RefCell<Wire>>>, output_wires: Vec<Rc<RefCell<Wire>>>) -> Self {
        XorGate {
            input_wires,
            output_wires,
        }
    }
}

impl GateTrait for XorGate {
    fn evaluate(&mut self) {
        let in1 = &mut self.input_wires[0].try_borrow_mut().unwrap();
        let in2 = &mut self.input_wires[1].try_borrow_mut().unwrap();
        let out = &mut self.output_wires[0].try_borrow_mut().unwrap();
        let u = in1.selector.as_mut().unwrap();
        let v = in2.selector.as_mut().unwrap();
        let w = *u ^ *v;
        out.selector = Some(w);
    }

    fn run_gate_on_inputs(&self, inputs: Vec<bool>) -> Vec<bool> {
        assert!(inputs.len() == 2);
        vec![inputs[0] ^ inputs[1]]
    }

    fn get_input_size(&self) -> usize {
        2
    }

    fn get_output_size(&self) -> usize {
        1
    }

    fn create_response_script(&self, lock_hash: [u8; 32]) -> ScriptBuf {
        let builder = Builder::new()
            .push_opcode(OP_SHA256)
            .push_slice(lock_hash)
            .push_opcode(OP_EQUALVERIFY);
        let builder = self.output_wires[0]
            .try_borrow()
            .unwrap()
            .add_bit_commitment_script(builder)
            .push_opcode(OP_TOALTSTACK);
        let builder = self.input_wires[1]
            .try_borrow()
            .unwrap()
            .add_bit_commitment_script(builder)
            .push_opcode(OP_TOALTSTACK);
        let builder = self.input_wires[0]
            .try_borrow()
            .unwrap()
            .add_bit_commitment_script(builder);
        builder
            .push_opcode(OP_FROMALTSTACK)
            .push_opcode(OP_NUMEQUAL)
            .push_opcode(OP_NOT)
            .push_opcode(OP_FROMALTSTACK)
            .push_opcode(OP_EQUALVERIFY)
            .into_script()
    }
}

#[cfg(test)]
mod tests {
    use std::vec;
    use super::*;
    use bitcoin::hashes::sha256;
    use bitcoin::hashes::Hash;
    use bitcoin::TapLeafHash;
    use bitcoin::Transaction;
    use bitcoin_scriptexec::*;
    use rand::Rng;

    fn check_exec(mut exec: Exec, correct_exec: bool) -> bool {
        let has_error = loop {
            if exec.exec_next().is_err() {
                // println!("error: {:?}", exec.exec_next().err());
                break true;
            }
        };
        let res = exec.result().unwrap().clone();
        println!("res: {:?}", res);
        if correct_exec {
            assert_eq!(res.error, None);
            false
        } else {
            assert!(has_error);
            true
        }
    }

    fn create_exec(script: &ScriptBuf, solution_preimages: Vec<Vec<u8>>) -> Exec {
        
        Exec::new(
            ExecCtx::Tapscript,
            Options::default(),
            TxTemplate {
                tx: Transaction {
                    version: bitcoin::transaction::Version::TWO,
                    lock_time: bitcoin::locktime::absolute::LockTime::ZERO,
                    input: vec![],
                    output: vec![],
                },
                prevouts: vec![],
                input_idx: 0,
                taproot_annex_scriptleaf: Some((TapLeafHash::all_zeros(), None)),
            },
            script.clone(),
            solution_preimages,
        )
        .expect("error creating exec")
    }

    fn get_preimage_index(input: bool) -> usize {
        if input {
            1
        } else {
            0
        }
    }

    fn generate_all_possibilities(num_inputs: usize) -> Vec<Vec<bool>> {
        let mut all_possibilities = vec![];
        for i in 0..2usize.pow(num_inputs as u32) {
            let mut input = vec![];
            for j in 0..num_inputs {
                input.push((i >> j) & 1 == 1);
            }
            all_possibilities.push(input);
        }
        all_possibilities
    }

    #[test]
    fn test_not_gate() {
        // get the input wire preimages, it should not be option, but a vector of preimages
        let input_wire_0 = Wire::new(0);
        let input_wire_0_preimages = input_wire_0.preimages.unwrap();
        let output_wire_0 = Wire::new(1);
        let output_wire_0_preimages = output_wire_0.preimages.unwrap();

        println!("input wire preimages: {:?}", input_wire_0_preimages);
        println!("output wire preimages: {:?}", output_wire_0_preimages);

        let not_gate = NotGate::new(
            vec![Rc::new(RefCell::new(input_wire_0))],
            vec![Rc::new(RefCell::new(output_wire_0))],
        );
        let all_possible_inputs = generate_all_possibilities(not_gate.input_wires.len());

        let mut rng = rand::thread_rng();
        let lock_preimage: [u8; 32] = rng.gen();
        let lock_hash = sha256::Hash::hash(&lock_preimage).to_byte_array();
        let script = not_gate.create_response_script(lock_hash);
        println!("script: {:?}", script);

        for input in all_possible_inputs.iter() {
            let input_preimage_index = get_preimage_index(input[0]);
            let gate_res = not_gate.run_gate_on_inputs(vec![input[0]]);
            let correct_output_preimage_index = get_preimage_index(gate_res[0]);

            let solution_preimages_1 = vec![
                input_wire_0_preimages[input_preimage_index]
                    .clone()
                    .to_vec(),
                output_wire_0_preimages[correct_output_preimage_index]
                    .clone()
                    .to_vec(),
                lock_preimage.to_vec(),
            ];

            println!("solution preimages: {:?}", solution_preimages_1.clone());

            let exec = create_exec(&script, solution_preimages_1);
            let has_error = check_exec(exec, true);
            assert!(!has_error);

            let incorrect_output_preimage_index = get_preimage_index(!gate_res[0]);
            let solution_preimages_2 = vec![
                input_wire_0_preimages[input_preimage_index]
                    .clone()
                    .to_vec(),
                output_wire_0_preimages[incorrect_output_preimage_index]
                    .clone()
                    .to_vec(),
                lock_preimage.to_vec(),
            ];
            let exec = create_exec(&script, solution_preimages_2);
            let has_error = check_exec(exec, false);
            assert!(has_error);
        }
    }

    fn test_gate(gate_name: &str) {
        let wire_0 = Wire::new(0);
        let wire_1 = Wire::new(1);
        let input_wires = vec![Rc::new(RefCell::new(wire_0.clone())), Rc::new(RefCell::new(wire_1.clone()))];
        let wire_2 = Wire::new(2);
        let output_wires = vec![Rc::new(RefCell::new(wire_2.clone()))];
        let gate: Box<dyn GateTrait> = match gate_name {
            "NotGate" => Box::new(NotGate::new(
                vec![Rc::new(RefCell::new(wire_0.clone()))],

                vec![Rc::new(RefCell::new(wire_2.clone()))],
            )),
            "AndGate" => Box::new(AndGate::new(
                vec![
                    Rc::new(RefCell::new(wire_0.clone())),
                    Rc::new(RefCell::new(wire_1.clone())),
                ],
                vec![Rc::new(RefCell::new(wire_2.clone()))],
            )),
            _ => Box::new(XorGate::new(
                vec![
                    Rc::new(RefCell::new(wire_0.clone())),
                    Rc::new(RefCell::new(wire_1.clone())),
                ],
                vec![Rc::new(RefCell::new(wire_2.clone()))],
            )),
        };
        let input_size = gate.get_input_size();
        let output_size = gate.get_output_size();
        let vec_input_wires = input_wires.clone()[0..input_size].to_vec();
        let vec_output_wires = output_wires.clone()[0..output_size].to_vec();
        let mut input_wire_preimages = vec![];
        let mut output_wire_preimages = vec![];

        for i in 0..input_size {
            input_wire_preimages.push(vec_input_wires[i].try_borrow().unwrap().preimages.unwrap());
        }
        for i in 0..output_size {
            output_wire_preimages
                .push(vec_output_wires[i].try_borrow().unwrap().preimages.unwrap());
        }

        println!("input wire preimages: {:?}", input_wire_preimages);
        println!("output wire preimages: {:?}", output_wire_preimages);

        let all_possible_inputs = generate_all_possibilities(input_size);
        let all_possible_outputs = generate_all_possibilities(output_size);

        println!("all possible inputs: {:?}", all_possible_inputs);
        println!("all possible outputs: {:?}", all_possible_outputs);

        let mut rng = rand::thread_rng();
        let lock_preimage: [u8; 32] = rng.gen();
        let lock_hash = sha256::Hash::hash(&lock_preimage).to_byte_array();
        let script = gate.create_response_script(lock_hash);

        println!("script: {:?}", script);

        for input in all_possible_inputs.iter() {
            let mut input_preimage_indices = vec![];
            for i in 0..input_size {
                input_preimage_indices.push(get_preimage_index(input[i]));
            }
            let gate_res = gate.run_gate_on_inputs(input.clone());

            let mut input_solution_preimages = vec![];

            for i in 0..input_size {
                input_solution_preimages
                    .push(input_wire_preimages[i][input_preimage_indices[i]].to_vec());
            }

            //do this with for so that all possibilities are covered
            for output in all_possible_outputs.iter() {
                let mut output_preimage_indices = vec![];
                for i in 0..output_size {
                    output_preimage_indices.push(get_preimage_index(output[i]));
                }

                let mut output_solution_preimages = vec![];

                for i in 0..output_size {
                    output_solution_preimages
                        .push(output_wire_preimages[i][output_preimage_indices[i]].to_vec());
                }

                let mut solution_preimages = input_solution_preimages.clone().to_vec();
                solution_preimages.extend(output_solution_preimages.clone().to_vec());
                solution_preimages.push(lock_preimage.clone().to_vec());

                println!("solution preimages: {:?}", solution_preimages.clone());

                let exec = create_exec(&script, solution_preimages);
                println!("gate_res: {:?}", gate_res);
                println!("output: {:?}", output);
                let mut compare_vectors = true;
                for i in 0..output_size {
                    if gate_res[i] != output[i] {
                        compare_vectors = false;
                        break;
                    }
                }
                println!("compare_vectors: {}", compare_vectors);
                println!("input preimage indices: {:?}", input_preimage_indices);
                println!("output preimage indices: {:?}", output_preimage_indices);
                let has_error = check_exec(exec, compare_vectors);
                println!("has_error: {}", has_error);
            }
        }
    }
    
    #[test]
    fn test_not_gate2() {
        test_gate("NotGate");
    }

    #[test]
    fn test_xor_gate() {
        test_gate("XorGate");
    }

    #[test]
    fn test_and_gate() {
        test_gate("AndGate");
    }
}