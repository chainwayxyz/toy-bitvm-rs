use crate::{traits::gate::GateTrait, wire::Wire};
use std::cell::RefCell;
use std::rc::Rc;

// Every gate has a type parameter COM, which is a bit commitment scheme which can be hash based or schnorr based.
// Every gate has an array of input wire pointers.
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
    fn create_challenge_script(&self) -> String {
        "NotGate".to_string()
    }

    fn evaluate(&mut self) {
        let in1 = &mut self.input_wires[0].try_borrow_mut().unwrap();
        let out = &mut self.output_wires[0].try_borrow_mut().unwrap();
        let u = in1.selector.as_mut().unwrap();
        let w = !*u;
        out.selector = Some(w);
    }

    fn set_input_wires(&mut self) {
        let in1 = &mut self.input_wires[0].try_borrow_mut().unwrap();
        let in2 = &mut self.input_wires[1].try_borrow_mut().unwrap();
        in1.selector = Some(true);
        in2.selector = Some(true);
    }

    fn print(&self) -> String {
        format!(
            "Gate[]: {:?}, {:?}, {:?}",
            self.input_wires[0], self.input_wires[1], self.output_wires[0]
        )
    }
}

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
    fn create_challenge_script(&self) -> String {
        "NotGate".to_string()
    }

    fn evaluate(&mut self) {
        let in1 = &mut self.input_wires[0].try_borrow_mut().unwrap();
        let in2 = &mut self.input_wires[1].try_borrow_mut().unwrap();
        let out = &mut self.output_wires[0].try_borrow_mut().unwrap();
        let u = in1.selector.as_mut().unwrap();
        let v = in2.selector.as_mut().unwrap();
        let w = *u && *v;
        out.selector = Some(w);
    }

    fn set_input_wires(&mut self) {}

    fn print(&self) -> String {
        format!(
            "Gate[]: {:?}, {:?}, {:?}",
            self.input_wires[0], self.input_wires[1], self.output_wires[0]
        )
    }
}

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
    fn create_challenge_script(&self) -> String {
        "NotGate".to_string()
    }

    fn evaluate(&mut self) {
        let in1 = &mut self.input_wires[0].try_borrow_mut().unwrap();
        let in2 = &mut self.input_wires[1].try_borrow_mut().unwrap();
        let out = &mut self.output_wires[0].try_borrow_mut().unwrap();
        let u = in1.selector.as_mut().unwrap();
        let v = in2.selector.as_mut().unwrap();
        let w = *u ^ *v;
        out.selector = Some(w);
    }

    fn set_input_wires(&mut self) {}

    fn print(&self) -> String {
        format!(
            "Gate[]: {:?}, {:?}, {:?}",
            self.input_wires[0], self.input_wires[1], self.output_wires[0]
        )
    }
}
