use std::vec::Vec;

pub struct HyperTensor {
    pub values: Vec<Vec<f64>>,
    pub m: f64,
    pub n: f64,
}

impl HyperTensor {
    pub fn new(vec: Vec<Vec<f64>>) -> Result<Self, &'static str> {
        if vec.len() < 2 {
            return Err("Vector has too few rows");
        }

        let cols = vec[0].len();

        if !vec.iter().all(|row| row.len() == cols) {
            return Err("Row size is not consistent");
        }

        if cols < 2 {
            return Err("Vector has too few columns");
        }

        let m = vec.len() as f64;
        let n = vec[0].len() as f64;

        Ok(Self { values: vec, m, n })
    }

    pub fn zeros(m: usize, n: usize) -> Result<Self, &'static str> {
        if n < 2 || m < 2 {
            return Err("Both dimensions must be at least 2 in size");
        }

        Self::new(vec![vec![0.0; n]; m])
    }
}
