use bitcoin::opcodes::all::{
    OP_AND, OP_EQUALVERIFY, OP_FROMALTSTACK, OP_NOT, OP_SHA256, OP_TOALTSTACK, OP_XOR,
};
use bitcoin::script::Builder;
use bitcoin::ScriptBuf;

use crate::transactions::add_bit_commitment_script;
use crate::wire::HashValue;
use crate::{traits::gate::GateTrait, wire::Wire};

use std::sync::{Arc, Mutex};

// Every gate has a type parameter COM, which is a bit commitment scheme which can be hash based or schnorr based.
// Every gate has an array of input wire pointers.
pub struct NotGate {
    pub input_wires: Vec<Arc<Mutex<Wire>>>,
    pub output_wires: Vec<Arc<Mutex<Wire>>>,
}

impl NotGate {
    pub fn new(input_wires: Vec<Arc<Mutex<Wire>>>, output_wires: Vec<Arc<Mutex<Wire>>>) -> Self {
        NotGate {
            input_wires,
            output_wires,
        }
    }
}

impl GateTrait for NotGate {
    fn evaluate(&mut self) {
        let in1 = &mut self.input_wires[0].lock().unwrap();
        let out = &mut self.output_wires[0].lock().unwrap();
        let u = in1.selector.as_mut().unwrap();
        let w = !*u;
        out.selector = Some(w);
    }

    fn create_response_script(&self, lock_hash: HashValue) -> ScriptBuf {
        let builder = Builder::new()
            .push_opcode(OP_SHA256)
            .push_slice(lock_hash)
            .push_opcode(OP_EQUALVERIFY);
        let builder = add_bit_commitment_script(
            self.output_wires[0].lock().unwrap().get_hash_pair(),
            builder,
        )
        .push_opcode(OP_TOALTSTACK);
        let builder =
            add_bit_commitment_script(self.input_wires[0].lock().unwrap().get_hash_pair(), builder);
        builder
            .push_opcode(OP_NOT)
            .push_opcode(OP_FROMALTSTACK)
            .push_opcode(OP_EQUALVERIFY)
            .into_script()
    }

    fn get_input_size(&self) -> usize {
        1
    }

    fn get_output_size(&self) -> usize {
        1
    }

    fn run_gate_on_inputs(&self, inputs: Vec<bool>) -> Vec<bool> {
        assert!(inputs.len() == 1);
        vec![!inputs[0]]
    }
}

pub struct AndGate {
    pub input_wires: Vec<Arc<Mutex<Wire>>>,
    pub output_wires: Vec<Arc<Mutex<Wire>>>,
}

impl AndGate {
    pub fn new(input_wires: Vec<Arc<Mutex<Wire>>>, output_wires: Vec<Arc<Mutex<Wire>>>) -> Self {
        AndGate {
            input_wires,
            output_wires,
        }
    }
}

impl GateTrait for AndGate {
    fn evaluate(&mut self) {
        let in1 = &mut self.input_wires[0].lock().unwrap();
        let in2 = &mut self.input_wires[1].lock().unwrap();
        let out = &mut self.output_wires[0].lock().unwrap();
        let u = in1.selector.as_mut().unwrap();
        let v = in2.selector.as_mut().unwrap();
        let w = *u && *v;
        out.selector = Some(w);
    }

    fn create_response_script(&self, lock_hash: HashValue) -> ScriptBuf {
        let builder = Builder::new()
            .push_opcode(OP_SHA256)
            .push_slice(lock_hash)
            .push_opcode(OP_EQUALVERIFY);
        let builder = add_bit_commitment_script(
            self.output_wires[0].lock().unwrap().get_hash_pair(),
            builder,
        )
        .push_opcode(OP_TOALTSTACK);
        let builder =
            add_bit_commitment_script(self.input_wires[0].lock().unwrap().get_hash_pair(), builder)
                .push_opcode(OP_TOALTSTACK);
        let builder =
            add_bit_commitment_script(self.input_wires[0].lock().unwrap().get_hash_pair(), builder);
        builder
            .push_opcode(OP_FROMALTSTACK)
            .push_opcode(OP_AND)
            .push_opcode(OP_FROMALTSTACK)
            .push_opcode(OP_EQUALVERIFY)
            .into_script()
    }

    fn get_input_size(&self) -> usize {
        2
    }

    fn get_output_size(&self) -> usize {
        1
    }

    fn run_gate_on_inputs(&self, inputs: Vec<bool>) -> Vec<bool> {
        assert!(inputs.len() == 2);
        vec![inputs[0] && inputs[1]]
    }
}

pub struct XorGate {
    pub input_wires: Vec<Arc<Mutex<Wire>>>,
    pub output_wires: Vec<Arc<Mutex<Wire>>>,
}

impl XorGate {
    pub fn new(input_wires: Vec<Arc<Mutex<Wire>>>, output_wires: Vec<Arc<Mutex<Wire>>>) -> Self {
        XorGate {
            input_wires,
            output_wires,
        }
    }
}

impl GateTrait for XorGate {
    fn evaluate(&mut self) {
        let in1 = &mut self.input_wires[0].lock().unwrap();
        let in2 = &mut self.input_wires[1].lock().unwrap();
        let out = &mut self.output_wires[0].lock().unwrap();
        let u = in1.selector.as_mut().unwrap();
        let v = in2.selector.as_mut().unwrap();
        let w = *u ^ *v;
        out.selector = Some(w);
    }

    fn create_response_script(&self, lock_hash: HashValue) -> ScriptBuf {
        let builder = Builder::new()
            .push_opcode(OP_SHA256)
            .push_slice(lock_hash)
            .push_opcode(OP_EQUALVERIFY);
        let builder = add_bit_commitment_script(
            self.output_wires[0].lock().unwrap().get_hash_pair(),
            builder,
        )
        .push_opcode(OP_TOALTSTACK);
        let builder =
            add_bit_commitment_script(self.input_wires[0].lock().unwrap().get_hash_pair(), builder)
                .push_opcode(OP_TOALTSTACK);
        let builder =
            add_bit_commitment_script(self.input_wires[0].lock().unwrap().get_hash_pair(), builder);
        builder
            .push_opcode(OP_FROMALTSTACK)
            .push_opcode(OP_XOR)
            .push_opcode(OP_FROMALTSTACK)
            .push_opcode(OP_EQUALVERIFY)
            .into_script()
    }

    fn get_input_size(&self) -> usize {
        2
    }

    fn get_output_size(&self) -> usize {
        1
    }

    fn run_gate_on_inputs(&self, inputs: Vec<bool>) -> Vec<bool> {
        assert!(inputs.len() == 2);
        vec![inputs[0] ^ inputs[1]]
    }
}

#[cfg(test)]
mod tests {
    use crate::wire::PreimageValue;

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

    fn test_gate(gate_name: &str) {

        let wire_0 = Wire::new(0);
        let wire_0_preimages = wire_0.preimages.unwrap();

        let wire_1 = Wire::new(1);
        let wire_1_preimages = wire_1.preimages.unwrap();

        let wire_2 = Wire::new(2);
        let wire_2_preimages = wire_2.preimages.unwrap();

        let input_wires = vec![
            Arc::new(Mutex::new(wire_0.clone())),
            Arc::new(Mutex::new(wire_1.clone())),
        ];
        let output_wires = vec![Arc::new(Mutex::new(wire_2.clone()))];

        let gate: Box<dyn GateTrait> = match gate_name {
            "NotGate" => Box::new(NotGate::new(
                vec![Arc::new(Mutex::new(wire_0.clone()))],
                vec![Arc::new(Mutex::new(wire_2.clone()))],
            )),
            "AndGate" => Box::new(AndGate::new(
                vec![
                    Arc::new(Mutex::new(wire_0.clone())),
                    Arc::new(Mutex::new(wire_1.clone())),
                ],
                vec![Arc::new(Mutex::new(wire_2.clone()))],
            )),
            _ => Box::new(AndGate::new(
                vec![
                    Arc::new(Mutex::new(wire_0.clone())),
                    Arc::new(Mutex::new(wire_1.clone())),
                ],
                vec![Arc::new(Mutex::new(wire_2.clone()))],
            )),
        };
        let input_size = gate.get_input_size();
        let output_size = gate.get_output_size();
        let mut input_wire_preimages = vec![];
        let mut output_wire_preimages = vec![];

        for i in 0..input_size {
           let mut guard = input_wires[i].clone().lock().expect("Failed to lock mutex");
           input_wire_preimages.push(guard.preimages.unwrap());
        }

        for i in 0..output_size {
            let mut guard = output_wires[i].clone().lock().expect("Failed to lock mutex");
            output_wire_preimages.push(guard.preimages.unwrap());

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
        //fix conversion from bool to u8, we do not need it anymore
        for input in all_possible_inputs.iter() {
            let mut input_preimage_indices = vec![];
            for i in 0..input_size {
                input_preimage_indices.push(get_preimage_index(input[i]));
            }
            let gate_res = gate.run_gate_on_inputs(input.clone());

            let mut input_solution_preimages = vec![];

            for i in 0..input_size {
                if input_preimage_indices[i] == 1 {
                    assert!(input_wire_preimages[i].one.len() == 32);
                    input_solution_preimages
                        .push(input_wire_preimages[i].one.clone().to_vec());
                } else {
                    assert!(input_wire_preimages[i].zero.len() == 32);
                    input_solution_preimages
                        .push(input_wire_preimages[i].zero.clone().to_vec());
                }
            }
            
            //do this with for so that all possibilities are covered
            for output in all_possible_outputs.iter() {
                let mut output_preimage_indices = vec![];
                for i in 0..output_size {
                    output_preimage_indices.push(get_preimage_index(output[i]));
                }

                let mut output_solution_preimages = vec![];

                for i in 0..output_size {
                    if output_preimage_indices[i] == 1 {
                        assert!(output_wire_preimages[i].one.len() == 32);
                        output_solution_preimages
                            .push(output_wire_preimages[i].one.clone().to_vec());
                    } else {
                        assert!(output_wire_preimages[i].zero.len() == 32);
                        output_solution_preimages
                            .push(output_wire_preimages[i].zero.clone().to_vec());
                    }
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
    fn test_not_gate() {
        let input_wire_0 = Wire::new(0);
        // get the input wire preimages, it should not be option, but a vector of preimages
        let input_wire_0_preimages = input_wire_0.preimages.unwrap();
        let output_wire_0 = Wire::new(1);
        let output_wire_0_preimages = output_wire_0.preimages.unwrap();

        let not_gate = NotGate::new(
            vec![Arc::new(Mutex::new(input_wire_0))],
            vec![Arc::new(Mutex::new(output_wire_0))],
        );

        let mut rng = rand::thread_rng();

        let lock_preimage: PreimageValue = rng.gen();

        let lock_hash = sha256::Hash::hash(&lock_preimage).to_byte_array();

        let script = not_gate.create_response_script(lock_hash);

        let solution_01_preimages = vec![
            input_wire_0_preimages.zero.clone().to_vec(),
            output_wire_0_preimages.one.clone().to_vec(),
            lock_preimage.to_vec(),
        ];
        let mut exec_01 = Exec::new(
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
            solution_01_preimages,
        )
        .expect("error creating exec");

        loop {
            if exec_01.exec_next().is_err() {
                println!("error: {:?}", exec_01.exec_next().err());
                break;
            }
        }

        let res = exec_01.result().unwrap().clone();
        println!("res: {:?}", res);

        assert_eq!(res.error, None);

        let solution_01_preimages = vec![
            input_wire_0_preimages.zero.clone().to_vec(),
            output_wire_0_preimages.zero.clone().to_vec(),
            lock_preimage.to_vec(),
        ];
        let mut exec_00 = Exec::new(
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
            script,
            solution_01_preimages,
        )
        .expect("error creating exec");

        let has_error = loop {
            if exec_00.exec_next().is_err() {
                println!("error: {:?}", exec_00.exec_next().err());
                break true;
            }
        };
        assert!(has_error);
    }
}
