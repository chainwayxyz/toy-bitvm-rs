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
