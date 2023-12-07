//use crate::wire::Wire;
//use std::rc::Rc;
//use std::cell::RefCell;

use bitcoin::ScriptBuf;

pub trait GateTrait {
    fn evaluate(&mut self);
    fn create_response_script(&self, lock_hash: [u8; 32]) -> ScriptBuf;
}
