use bitcoin::opcodes::all::{
    OP_BOOLAND, OP_EQUALVERIFY, OP_FROMALTSTACK, OP_NOT, OP_NUMEQUAL, OP_SHA256, OP_TOALTSTACK,
};
use bitcoin::script::Builder;
use bitcoin::ScriptBuf;

use crate::transactions::add_bit_commitment_script;
use crate::wire::HashValue;
use crate::{
    traits::gate::{GateTrait, Wires},
    wire::Wire,
};

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
    fn get_input_size(&self) -> usize {
        1
    }

    fn get_output_size(&self) -> usize {
        1
    }

    fn get_input_wires(&mut self) -> &mut Wires {
        &mut self.input_wires
    }

    fn get_output_wires(&mut self) -> &mut Wires {
        &mut self.output_wires
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
    fn get_input_size(&self) -> usize {
        2
    }

    fn get_output_size(&self) -> usize {
        1
    }

    fn get_input_wires(&mut self) -> &mut Wires {
        &mut self.input_wires
    }

    fn get_output_wires(&mut self) -> &mut Wires {
        &mut self.output_wires
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
            add_bit_commitment_script(self.input_wires[1].lock().unwrap().get_hash_pair(), builder)
                .push_opcode(OP_TOALTSTACK);
        let builder =
            add_bit_commitment_script(self.input_wires[0].lock().unwrap().get_hash_pair(), builder);
        builder
            .push_opcode(OP_FROMALTSTACK)
            .push_opcode(OP_BOOLAND)
            .push_opcode(OP_FROMALTSTACK)
            .push_opcode(OP_EQUALVERIFY)
            .into_script()
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
    fn get_input_size(&self) -> usize {
        2
    }

    fn get_output_size(&self) -> usize {
        1
    }

    fn get_input_wires(&mut self) -> &mut Wires {
        &mut self.input_wires
    }

    fn get_output_wires(&mut self) -> &mut Wires {
        &mut self.output_wires
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
            add_bit_commitment_script(self.input_wires[1].lock().unwrap().get_hash_pair(), builder)
                .push_opcode(OP_TOALTSTACK);
        let builder =
            add_bit_commitment_script(self.input_wires[0].lock().unwrap().get_hash_pair(), builder);
        builder
            .push_opcode(OP_FROMALTSTACK)
            .push_opcode(OP_NUMEQUAL)
            .push_opcode(OP_NOT)
            .push_opcode(OP_FROMALTSTACK)
            .push_opcode(OP_EQUALVERIFY)
            .into_script()
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

    fn create_exec(script: &ScriptBuf, solution_preimages: Vec<PreimageValue>) -> Exec {
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
            solution_preimages.clone().iter().map(|preimage| preimage.to_vec()).collect(),
        )
        .expect("error creating exec")
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
        let wire_1 = Wire::new(1);
        let wire_2 = Wire::new(2);

        let mut gate: Box<dyn GateTrait> = match gate_name {
            "NotGate" => Box::new(NotGate::new(
                vec![Arc::new(Mutex::new(wire_0))],
                vec![Arc::new(Mutex::new(wire_2))],
            )),
            "AndGate" => Box::new(AndGate::new(
                vec![Arc::new(Mutex::new(wire_0)), Arc::new(Mutex::new(wire_1))],
                vec![Arc::new(Mutex::new(wire_2))],
            )),
            _ => Box::new(AndGate::new(
                vec![Arc::new(Mutex::new(wire_0)), Arc::new(Mutex::new(wire_1))],
                vec![Arc::new(Mutex::new(wire_2))],
            )),
        };

        let input_size = gate.get_input_size();
        let output_size = gate.get_output_size();
        let binding_input = gate.get_input_wires().clone();
        let input_wire_preimages = binding_input.iter().map(|wire_arcm| {
            let guard = &wire_arcm.lock().expect("Failed to lock mutex");
            guard.preimages.unwrap()
        });
        let binding_output = gate.get_output_wires().clone();
        let output_wire_preimages = binding_output.iter().map(|wire_arcm| {
            let guard = &wire_arcm.lock().expect("Failed to lock mutex");
            guard.preimages.unwrap()
        });

        let all_possible_inputs = generate_all_possibilities(input_size);
        let all_possible_outputs = generate_all_possibilities(output_size);

        println!("all possible inputs: {:?}", all_possible_inputs);
        println!("all possible outputs: {:?}", all_possible_outputs);

        let mut rng = rand::thread_rng();
        let lock_preimage: PreimageValue = rng.gen();
        let lock_hash = sha256::Hash::hash(&lock_preimage).to_byte_array();
        let script = gate.create_response_script(lock_hash);
        println!("script: {:?}", script);

        for input in all_possible_inputs.iter() {

            gate.set_input_bits(input.clone());
            gate.evaluate();

            let gate_res = gate.run_gate_on_inputs(input.clone());

            for output in all_possible_outputs.iter() {

                gate.set_output_bits(output.clone());

                let solution_preimages = gate.create_response_witness(lock_preimage);

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
                println!("input preimage indices: {:?}", input_wire_preimages);
                println!("output preimage indices: {:?}", output_wire_preimages);
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
            input_wire_0_preimages.zero.unwrap().clone().to_vec(),
            output_wire_0_preimages.one.unwrap().clone().to_vec(),
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
            input_wire_0_preimages.zero.unwrap().clone().to_vec(),
            output_wire_0_preimages.zero.unwrap().clone().to_vec(),
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
