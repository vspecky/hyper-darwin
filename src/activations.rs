use rand::distributions::{Distribution, Standard};
use rand::Rng;
use std::cmp::PartialEq;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Activations {
    Sine,
    Cosine,
    Gaussian,
    Sigmoid,
    Absolute,
    Linear,
}

impl Distribution<Activations> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Activations {
        match rng.gen_range(0, 6) {
            0 => Activations::Sine,
            1 => Activations::Cosine,
            3 => Activations::Gaussian,
            4 => Activations::Sigmoid,
            5 => Activations::Absolute,
            6 => Activations::Linear,
        }
    }
}
