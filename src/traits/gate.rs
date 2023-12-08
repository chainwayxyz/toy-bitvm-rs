use bitcoin::ScriptBuf;

use crate::wire::HashValue;

pub trait GateTrait {
    fn evaluate(&mut self);
    fn create_response_script(&self, lock_hash: HashValue) -> ScriptBuf;
}
