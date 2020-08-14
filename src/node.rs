use crate::activations::Activations;
use std::fmt;

pub struct Node {
    pub innov: u32,
    pub activation: Activations,
    pub x: f64,
    pub y: f64,
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Node({})", self.innov)
    }
}

impl std::cmp::PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.innov == other.innov
    }
}

impl Node {
    pub fn new(innov: u32, x: f64, y: f64, activation: Activations) -> Self {
        Self {
            innov,
            x,
            y,
            activation,
        }
    }

    pub fn activate(&self, val: f64) -> f64 {
        if self.x == 0. {
            return val;
        }

        match self.activation {
            Activations::Linear => val,
            Activations::Absolute => val.abs(),
            Activations::Sigmoid => 1. / (1. + (-4.9 * val).exp()),
            Activations::Sine => val.sin(),
            Activations::Cosine => val.cos(),
            Activations::Gaussian => (-(val.powi(2) / 2.)).exp(),
        }
    }
}

impl std::clone::Clone for Node {
    fn clone(&self) -> Self {
        Self {
            innov: self.innov,
            activation: self.activation,
            x: self.x,
            y: self.y,
        }
    }
}
