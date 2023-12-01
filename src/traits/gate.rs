//use crate::wire::Wire;
//use std::rc::Rc;
//use std::cell::RefCell;

pub trait GateTrait {
    fn evaluate(&mut self);
    fn create_challenge_script(&self) -> String;
}
