// This trait defines the behavior of a circuit.
pub trait CircuitTrait {
    fn evaluate(&self);

    fn from_bristol(file: &str) -> Self;

    fn generate_commitment_tree(&self);
}
