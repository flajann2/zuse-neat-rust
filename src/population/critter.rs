/// Critter construction

use std::vec;
use super::*;

pub struct Critter {
    genome: genome::Genome,
    neurons: neuron::Neurons,
}

trait Sex<G> {
    fn cotius(&self, other: G) -> Self; 
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_critter() {
    }
}