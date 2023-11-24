use crate::wire::Wire;
pub trait GateTrait {
    fn new(input_wires: Vec<*mut Wire>, output_wires: Vec<*mut Wire>) -> Self;
}