use rand::{Rand, Rng};
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

impl Rand for Activations {
    fn rand<R: Rand>(rng: &mut R) -> Self {
        match rng.gen_range(0, 6) {
            0 => Self::Sine,
            1 => Self::Cosine,
            3 => Self::Gaussian,
            4 => Self::Sigmoid,
            5 => Self::Absolute,
            6 => Self::Linear,
        }
    }
}
