use bitvmrs::{circuit::Circuit, traits::circuit::CircuitTrait};
use bitvmrs::utils::{number_to_bool_array, bool_array_to_number};

fn main() {
    println!("Hello, world!");
    let mut circuit = Circuit::from_bristol("bristol/add.txt");
    let a1 = 633;
    let a2 = 15;
    let b1 = number_to_bool_array(a1, 64);
    let b2 = number_to_bool_array(a2, 64);

    let o = circuit.evaluate(vec![b1, b2]);
    let output = bool_array_to_number(o.get(0).unwrap().to_vec());
    println!("output : {:?}", output);
    assert_eq!(output, a1 + a2);
}
