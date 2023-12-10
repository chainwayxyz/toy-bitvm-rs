use bitcoin::opcodes::all::{
    OP_BOOLAND, OP_EQUALVERIFY, OP_FROMALTSTACK, OP_NOT, OP_NUMEQUAL, OP_SHA256, OP_TOALTSTACK, OP_BOOLOR, OP_2DUP,
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
        let builder = add_bit_commitment_script(
            self.output_wires[0].lock().unwrap().get_hash_pair(),
            Builder::new(),
        )
        .push_opcode(OP_TOALTSTACK);
        let builder =
            add_bit_commitment_script(self.input_wires[0].lock().unwrap().get_hash_pair(), builder);
        builder
            .push_opcode(OP_NOT)
            .push_opcode(OP_FROMALTSTACK)
            .push_opcode(OP_EQUALVERIFY)
            .push_opcode(OP_SHA256)
            .push_slice(lock_hash)
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
        let builder = add_bit_commitment_script(
            self.output_wires[0].lock().unwrap().get_hash_pair(),
            Builder::new(),
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
            .push_opcode(OP_SHA256)
            .push_slice(lock_hash)
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
        let builder = add_bit_commitment_script(
            self.output_wires[0].lock().unwrap().get_hash_pair(),
            Builder::new(),
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
            .push_opcode(OP_SHA256)
            .push_slice(lock_hash)
            .push_opcode(OP_EQUALVERIFY)
            .into_script()
    }

    fn run_gate_on_inputs(&self, inputs: Vec<bool>) -> Vec<bool> {
        assert!(inputs.len() == 2);
        vec![inputs[0] ^ inputs[1]]
    }
}

pub struct OrGate {
    pub input_wires: Vec<Arc<Mutex<Wire>>>,
    pub output_wires: Vec<Arc<Mutex<Wire>>>,
}

impl OrGate {
    pub fn new(input_wires: Vec<Arc<Mutex<Wire>>>, output_wires: Vec<Arc<Mutex<Wire>>>) -> Self {
        OrGate {
            input_wires,
            output_wires,
        }
    }
}

impl GateTrait for OrGate {

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
        let builder = add_bit_commitment_script(
            self.output_wires[0].lock().unwrap().get_hash_pair(),
            Builder::new(),
        )
        .push_opcode(OP_TOALTSTACK);
        let builder =
            add_bit_commitment_script(self.input_wires[1].lock().unwrap().get_hash_pair(), builder)
                .push_opcode(OP_TOALTSTACK);
        let builder =
            add_bit_commitment_script(self.input_wires[0].lock().unwrap().get_hash_pair(), builder);
        builder
            .push_opcode(OP_FROMALTSTACK)
            .push_opcode(OP_BOOLOR)
            .push_opcode(OP_FROMALTSTACK)
            .push_opcode(OP_EQUALVERIFY)
            .push_opcode(OP_SHA256)
            .push_slice(lock_hash)
            .push_opcode(OP_EQUALVERIFY)
            .into_script()
    }

    fn run_gate_on_inputs(&self, inputs: Vec<bool>) -> Vec<bool> {
        assert!(inputs.len() == 2);
        vec![inputs[0] || inputs[1]]
    }
}

pub struct BitAdditionGate {
    pub input_wires: Vec<Arc<Mutex<Wire>>>,
    pub output_wires: Vec<Arc<Mutex<Wire>>>,
}

impl BitAdditionGate {
    pub fn new(input_wires: Vec<Arc<Mutex<Wire>>>, output_wires: Vec<Arc<Mutex<Wire>>>) -> Self {
        BitAdditionGate {
            input_wires,
            output_wires,
        }
    }
}

impl GateTrait for BitAdditionGate {
    fn get_input_size(&self) -> usize {
        2
    }

    fn get_output_size(&self) -> usize {
        2
    }

    fn get_input_wires(&mut self) -> &mut Wires {
        &mut self.input_wires
    }

    fn get_output_wires(&mut self) -> &mut Wires {
        &mut self.output_wires
    }

    fn create_response_script(&self, lock_hash: HashValue) -> ScriptBuf {
        let builder = add_bit_commitment_script(
            self.output_wires[1].lock().unwrap().get_hash_pair(),
            Builder::new(),
        )
        .push_opcode(OP_TOALTSTACK);
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
            .push_opcode(OP_2DUP)
            .push_opcode(OP_NUMEQUAL)
            .push_opcode(OP_NOT)
            .push_opcode(OP_FROMALTSTACK)
            .push_opcode(OP_EQUALVERIFY)
            .push_opcode(OP_BOOLAND)
            .push_opcode(OP_FROMALTSTACK)
            .push_opcode(OP_EQUALVERIFY)
            .push_opcode(OP_SHA256)
            .push_slice(lock_hash)
            .push_opcode(OP_EQUALVERIFY)
            .into_script()
    }

    fn run_gate_on_inputs(&self, inputs: Vec<bool>) -> Vec<bool> {
        assert!(inputs.len() == 2);
        let sum = inputs[0] ^ inputs[1];
        let carry = inputs[0] && inputs[1];
        vec![sum, carry]
    }
}

macro_rules! create_gate_without_wires {
    ($gate_type:ty, $input_wires:expr, $output_wires:expr) => {{
        if let (Some(input_wires), Some(output_wires)) =
            ($input_wires.as_ref(), $output_wires.as_ref())
        {
            return Box::new(<$gate_type>::new(input_wires.clone(), output_wires.clone()));
        }
        let dummy_gate = <$gate_type>::new(vec![], vec![]);
        let input_wires: Vec<_> = (0..dummy_gate.get_input_size())
            .map(|_| Arc::new(Mutex::new(Wire::new(0))))
            .collect();
        let output_wires: Vec<_> = (0..dummy_gate.get_output_size())
            .map(|_| Arc::new(Mutex::new(Wire::new(0))))
            .collect();
        Box::new(<$gate_type>::new(input_wires, output_wires))
    }};
}

pub fn create_gate(
    gate_name: &str,
    input_wires: Option<Vec<Arc<Mutex<Wire>>>>,
    output_wires: Option<Vec<Arc<Mutex<Wire>>>>,
) -> Box<dyn GateTrait + std::marker::Send> {
    match gate_name {
        "not" => create_gate_without_wires!(NotGate, &input_wires, &output_wires),
        "xor" => create_gate_without_wires!(XorGate, &input_wires, &output_wires),
        "and" => create_gate_without_wires!(AndGate, &input_wires, &output_wires),
        "or" => create_gate_without_wires!(OrGate, &input_wires, &output_wires),
        "bit_addition" => create_gate_without_wires!(BitAdditionGate, &input_wires, &output_wires),
        _ => panic!("Invalid gate name"),
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

    fn check_exec(mut exec: Exec, correct_exec: bool) {
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
        } else {
            assert!(has_error);
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
            solution_preimages
                .iter()
                .map(|preimage| preimage.to_vec())
                .collect(),
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
        let mut gate: Box<dyn GateTrait> = create_gate(gate_name, None, None);
        let all_possible_inputs = generate_all_possibilities(gate.get_input_size());
        let all_possible_outputs = generate_all_possibilities(gate.get_output_size());

        let mut rng = rand::thread_rng();
        let lock_preimage: PreimageValue = rng.gen();
        let lock_hash = sha256::Hash::hash(&lock_preimage).to_byte_array();
        let script = gate.create_response_script(lock_hash);

        for input in all_possible_inputs.iter() {
            gate.set_input_bits(input.clone());
            let gate_res = gate.run_gate_on_inputs(input.clone());
            for output in all_possible_outputs.iter() {
                gate.set_output_bits(output.clone());
                let solution_preimages = gate.create_response_witness(lock_preimage);
                let exec = create_exec(&script, solution_preimages);
                let compare_vectors = gate_res.iter().eq(output.iter());
                check_exec(exec, compare_vectors);
            }
        }
    }

    #[test]
    fn test_not_gate() {
        test_gate("not");
    }

    #[test]
    fn test_xor_gate() {
        test_gate("xor");
    }

    #[test]
    fn test_and_gate() {
        test_gate("and");
    }

    #[test]
    fn test_or_gate() {
        test_gate("or");
    }

    #[test]
    fn test_bit_addition_gate() {
        test_gate("bit_addition")
    }
}
