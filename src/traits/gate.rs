//use crate::wire::Wire;
//use std::rc::Rc;
//use std::cell::RefCell;

pub trait GateTrait {
    fn create_challenge_script(&self) -> String;
    fn evaluate(&mut self);
    fn set_input_wires(&mut self);
    fn print(&self) -> String;
}