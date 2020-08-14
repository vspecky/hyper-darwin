pub struct Settings {
    pub pop_size: u32,
    pub inputs: u32,
    pub outputs: u32,

    pub conn_mut_rate: f64,
    pub node_mut_rate: f64,
    pub wt_mut_rate: f64,
    pub wt_shift_rate: f64,

    pub off_gene_on_rate: f64,
    pub off_in_both_on_rate: f64,
    pub only_mut_rate: f64,

    pub disjoint_coeff: f64,
    pub excess_coeff: f64,
    pub weight_coeff: f64,
    pub activation_coeff: f64,
    pub speciation_threshold: f64,
    pub allowed_stagnancy: u32,
}

impl Settings {
    pub fn new(inputs: u32, outputs: u32, pop_size: u32) -> Self {
        Self {
            inputs,
            outputs,
            pop_size,
            conn_mut_rate: 0.05,
            node_mut_rate: 0.03,
            wt_mut_rate: 0.8,
            wt_shift_rate: 0.9,
            off_gene_on_rate: 0.25,
            off_in_both_on_rate: 0.01,
            only_mut_rate: 0.25,
            disjoint_coeff: 1.,
            excess_coeff: 1.,
            activation_coeff: 1.,
            weight_coeff: 0.4,
            speciation_threshold: 3.,
            allowed_stagnancy: 15,
        }
    }

    pub fn conn_mut_rate(mut self, rate: f64) -> Self {
        self.conn_mut_rate = rate;
        self
    }

    pub fn node_mut_rate(mut self, rate: f64) -> Self {
        self.node_mut_rate = rate;
        self
    }

    pub fn wt_mut_rate(mut self, rate: f64) -> Self {
        self.wt_mut_rate = rate;
        self
    }

    pub fn wt_shift_rate(mut self, rate: f64) -> Self {
        self.wt_shift_rate = rate;
        self
    }

    pub fn off_gene_on_rate(mut self, rate: f64) -> Self {
        self.off_gene_on_rate = rate;
        self
    }

    pub fn off_in_both_on_rate(mut self, rate: f64) -> Self {
        self.off_in_both_on_rate = rate;
        self
    }

    pub fn only_mut_rate(mut self, rate: f64) -> Self {
        self.only_mut_rate = rate;
        self
    }

    pub fn disjoint_coeff(mut self, coeff: f64) -> Self {
        self.disjoint_coeff = coeff;
        self
    }

    pub fn excess_coeff(mut self, coeff: f64) -> Self {
        self.excess_coeff = coeff;
        self
    }

    pub fn activation_coeff(mut self, coeff: f64) -> Self {
        self.activation_coeff = coeff;
        self
    }

    pub fn weight_coeff(mut self, coeff: f64) -> Self {
        self.weight_coeff = coeff;
        self
    }

    pub fn speciation_threshold(mut self, threshold: f64) -> Self {
        self.speciation_threshold = threshold;
        self
    }

    pub fn allowed_stagnancy(mut self, stagnancy: u32) -> Self {
        self.allowed_stagnancy = stagnancy;
        self
    }
}
