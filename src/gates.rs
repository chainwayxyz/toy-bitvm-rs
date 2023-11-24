use crate::traits::{bit_commitment::BitCommitmentTrait, gate::GateTrait};

pub struct NotGate<COM: BitCommitmentTrait> {
    pub input: COM,
    pub output: COM,
}

impl<COM> GateTrait<COM> for NotGate<COM> where COM: BitCommitmentTrait {
    
}
