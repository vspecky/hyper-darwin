use crate::activations::Activations;
use crate::connection::Connection;
use crate::history::History;
use crate::hyper_tensor::HyperTensor;
use crate::node::Node;
use crate::settings::HyperSettings;
use crate::settings::Settings;

use rand::prelude::*;
use rand::seq::SliceRandom;
use rand::thread_rng;

use std::clone::Clone;
use std::collections::HashMap;
use std::fmt;
use std::vec::Vec;

// Main Genome Class
pub struct Genome {
    inputs: u32,                // Number of Inputs
    outputs: u32,               // Number of Outputs
    pub nodes: Vec<Node>,       // Vector of Nodes
    pub conns: Vec<Connection>, // Vector of Connections
    pub fitness: f64,           // Fitness of this Genome
}

impl fmt::Debug for Genome {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut res = String::from("Genome {\n    Nodes {");

        for node in &self.nodes {
            res += &format!("\n        {:?},", node);
        }

        res += "\n    }\n    Conn {";

        for conn in &self.conns {
            res += &format!("\n        {:?},", conn);
        }

        res += "\n    }\n}";

        write!(f, "{}", res)
    }
}

impl Genome {
    pub fn new(inputs: u32, outputs: u32, crossover: bool) -> Self {
        let mut genome = Self {
            inputs,
            outputs,
            nodes: Vec::with_capacity((inputs + outputs + 1) as usize),
            conns: Vec::with_capacity(((inputs + 1) * outputs) as usize),
            fitness: 0.,
        };

        if crossover {
            return genome;
        }

        let mut rng = thread_rng();

        let mut dy = 1. / (inputs + 1) as f64;
        let mut dy_curr = dy;

        for i in 1..=(inputs + 1) {
            genome
                .nodes
                .push(Node::new(i, 0., dy_curr, rng.gen::<Activations>()));
            dy_curr += dy;
        }

        dy = 1. / (outputs + 1) as f64;
        dy_curr = dy;

        for i in (inputs + 2)..(inputs + outputs + 2) {
            genome
                .nodes
                .push(Node::new(i, 1., dy_curr, rng.gen::<Activations>()));
            dy_curr += dy;
        }

        let mut ctr = 1;
        for i in 0..(inputs + 1) as usize {
            let from = genome.nodes[i].innov;
            for o in (inputs + 1) as usize..genome.nodes.len() {
                let to = genome.nodes[o].innov;
                genome
                    .conns
                    .push(Connection::new(ctr, from, to, rng.gen::<f64>(), true));

                ctr += 1;
            }
        }

        genome
    }

    pub fn add_fitness(&mut self, fit: f64) {
        let fitness = self.fitness + fit;

        self.fitness = if fitness < 0. { 0. } else { fitness };
    }

    pub fn feed_forward(&self, input: &Vec<f64>) -> Result<f64, &'static str> {
        if input.len() != self.inputs as usize {
            return Err("Provided input size doesn't match Genome input size");
        }

        let mut node_vals = HashMap::<u32, f64>::new();

        let mut i = 1;
        for val in input {
            node_vals.insert(i, *val);
            i += 1;
        }

        node_vals.insert(self.inputs + 1, 1.);

        for node in self.nodes.iter() {
            let from_val = match node_vals.get(&node.innov) {
                Some(v) => *v,
                None => return Err("No val"),
            };

            let feed_forward_val = node.activate(from_val);

            for conn in self.conns.iter().filter(|&c| c.from == node.innov) {
                let to_val = node_vals.entry(conn.to).or_insert(0.);
                if !conn.enabled {
                    continue;
                }
                *to_val += feed_forward_val * conn.weight;
            }
        }

        Ok(((1. / (1. + (*node_vals.get(&(self.inputs + 2)).unwrap() * -4.9).exp())) - 0.5) * 2.)
    }

    pub fn hyper_feed_forward<T>(
        &self,
        input: HyperTensor,
        sets: &HyperSettings,
        third_param_fn: Option<T>,
    ) -> Result<Vec<Vec<f64>>, &'static str>
    where
        T: Fn(f64, f64) -> f64,
    {
        if self.inputs == 4 && third_param_fn.is_some() {
            return Err("Didn't expect a Third Parameter Function");
        } else if self.inputs == 6 && third_param_fn.is_none() {
            return Err("Expected a Third Parameter Function. Got none");
        }

        let dy = (input.m - 1.) / 2.;
        let dx = (input.n - 1.) / 2.;

        let x_dir = (0..input.n as u64)
            .map(|v| -1.0 + v as f64 * dx)
            .collect::<Vec<f64>>();

        let y_dir = (0..input.m as u64)
            .map(|v| -1.0 + v as f64 * dy)
            .collect::<Vec<f64>>();

        let mut res = HyperTensor::zeros(input.m as usize, input.n as usize).unwrap();

        if self.inputs == 6 {
            let tpfn = third_param_fn.unwrap();

            for (&y1, row_inp) in y_dir.iter().zip(input.values.iter()) {
                for (&x1, val_inp) in x_dir.iter().zip(row_inp.iter()) {
                    for (&y2, row_out) in y_dir.iter().zip(res.values.iter_mut()) {
                        for (&x2, val_out) in x_dir.iter().zip(row_out.iter_mut()) {
                            let inp = vec![x1, y1, tpfn(x1, y1), x2, y2, tpfn(x2, y2)];

                            let cppn_out = self.feed_forward(&inp).unwrap();

                            *val_out += sets.scaled_weight(cppn_out) * val_inp;
                        }
                    }
                }
            }
        } else {
            for (&y1, row_inp) in y_dir.iter().zip(input.values.iter()) {
                for (&x1, val_inp) in x_dir.iter().zip(row_inp.iter()) {
                    for (&y2, row_out) in y_dir.iter().zip(res.values.iter_mut()) {
                        for (&x2, val_out) in x_dir.iter().zip(row_out.iter_mut()) {
                            let inp = vec![x1, y1, x2, y2];

                            let cppn_out = self.feed_forward(&inp).unwrap();

                            *val_out += sets.scaled_weight(cppn_out) * val_inp;
                        }
                    }
                }
            }
        }

        Ok(res.values)
    }

    pub fn mutate(&mut self, hist: &mut History, sets: &Settings) {
        let mut rng = thread_rng();

        self.conns.iter_mut().for_each(|c| {
            if rng.gen::<f64>() < sets.wt_mut_rate {
                c.mutate_weight(sets);
            }
        });

        if rng.gen::<f64>() < sets.conn_mut_rate {
            self.add_conn(hist);
        }

        if rng.gen::<f64>() < sets.node_mut_rate {
            self.add_node(hist);
        }

        self.conns.sort_unstable_by(|a, b| a.innov.cmp(&b.innov));
    }

    fn add_conn(&mut self, hist: &mut History) {
        let mut rng = thread_rng();

        let from_node_pool = self
            .nodes
            .iter()
            .filter(|node| {
                if node.x == 1. {
                    return false;
                }

                let to_nodes = self
                    .nodes
                    .iter()
                    .filter(|n| {
                        n.x > node.x
                            && self
                                .conns
                                .iter()
                                .find(|c| c.from == node.innov && c.to == n.innov)
                                .is_none()
                    })
                    .collect::<Vec<&Node>>();

                to_nodes.len() > 0
            })
            .collect::<Vec<&Node>>();

        if from_node_pool.len() == 0 {
            return;
        }

        let from_node = from_node_pool.choose(&mut rng).unwrap();

        let to_node_pool = self
            .nodes
            .iter()
            .filter(|n| {
                if n.x <= from_node.x {
                    return false;
                }

                self.conns
                    .iter()
                    .find(|c| c.from == from_node.innov && c.to == n.innov)
                    .is_none()
            })
            .collect::<Vec<&Node>>();

        let to_node = to_node_pool.choose(&mut rng).unwrap();

        let innov = hist.mutate_conn(from_node, to_node);

        let new_conn = Connection::new(
            innov,
            from_node.innov,
            to_node.innov,
            rng.gen::<f64>(),
            true,
        );

        self.conns.push(new_conn);
    }

    fn add_node(&mut self, hist: &mut History) {
        let mut rng = thread_rng();

        let conn_to_mutate = self.conns.iter_mut().choose(&mut rng).unwrap();

        let details = hist.mutate_node(&conn_to_mutate);

        let from_node = self
            .nodes
            .iter()
            .find(|n| n.innov == conn_to_mutate.from)
            .unwrap();
        let to_node = self
            .nodes
            .iter()
            .find(|n| n.innov == conn_to_mutate.to)
            .unwrap();

        let x = (from_node.x + to_node.x) / 2.;
        let y = (from_node.y + to_node.y) / 2.;

        let new_node = Node::new(details.node, x, y, rand::random::<Activations>());
        let in_conn = Connection::new(details.in_conn, from_node.innov, new_node.innov, 1., true);

        let out_conn = Connection::new(
            details.out_conn,
            new_node.innov,
            to_node.innov,
            conn_to_mutate.weight,
            true,
        );

        conn_to_mutate.disable();
        self.nodes.push(new_node);
        self.conns.push(in_conn);
        self.conns.push(out_conn);

        self.nodes
            .sort_unstable_by(|a, b| a.x.partial_cmp(&b.x).unwrap());
    }

    pub fn crossover(parent1: &Self, parent2: &Self, sets: &Settings) -> Self {
        let (male, female) = if parent1.fitness >= parent2.fitness {
            (parent1, parent2)
        } else {
            (parent2, parent1)
        };

        let mut offspring_genes = Vec::<Connection>::with_capacity(male.conns.len());

        let mut rng = thread_rng();

        let mut f_genes = HashMap::<u32, &Connection>::new();

        female.conns.iter().for_each(|c| {
            f_genes.insert(c.innov, c);
        });

        for conn in &male.conns {
            if f_genes.contains_key(&conn.innov) {
                let f_gene = *f_genes.get(&conn.innov).unwrap();
                let mut gene = if rng.gen::<f64>() < 0.5 {
                    f_gene.clone()
                } else {
                    conn.clone()
                };

                let m_e = conn.enabled;
                let f_e = f_gene.enabled;

                if (!f_e && m_e) || (!m_e && f_e) {
                    if rng.gen::<f64>() < sets.off_gene_on_rate {
                        gene.enable();
                    } else {
                        gene.disable();
                    }
                } else if !f_e && !m_e {
                    if rng.gen::<f64>() < sets.off_in_both_on_rate {
                        gene.enable();
                    } else {
                        gene.disable();
                    }
                }

                offspring_genes.push(gene);
            } else {
                offspring_genes.push(conn.clone());
            }
        }

        let mut offspring = Self::new(male.inputs, male.outputs, true);
        offspring.conns = offspring_genes;
        offspring.nodes = male.nodes.clone();

        offspring
    }
}

impl Clone for Genome {
    fn clone(&self) -> Self {
        Self {
            inputs: self.inputs,
            outputs: self.outputs,
            nodes: self.nodes.clone(),
            conns: self.conns.clone(),
            fitness: self.fitness,
        }
    }
}
