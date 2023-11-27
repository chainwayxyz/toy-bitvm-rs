use bitvmrs::{circuit::Circuit, traits::circuit::CircuitTrait};

fn main() {
    println!("Hello, world!");
    let circuit = Circuit::from_bristol("bristol/add.txt");
    println!("{}", circuit.input_sizes[0]);
}
