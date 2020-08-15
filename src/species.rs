use crate::genome::Genome;
use crate::settings::Settings;

use rand::seq::SliceRandom;
use rand::thread_rng;
use rand::Rng;

use std::fmt;
use std::vec::Vec;

pub struct Species {
    pub genomes: Vec<Genome>,
    max_fitness: f64,
    pub avg_fitness: f64,
    pub stagnancy: u32,
    representative: Genome,
    pub assigned_offspring: usize,
}

impl fmt::Debug for Species {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut res = String::from(&format!("Genomes: {}\n", self.genomes.len()));
        res += &format!("max_fitness: {}\n", self.max_fitness);
        res += &format!("representative: {:?}", self.representative);

        write!(f, "{}", res)
    }
}

impl Species {
    pub fn new(head: Genome) -> Self {
        let max_fitness = head.fitness;
        let avg_fitness = head.fitness;
        let repr = head.clone();

        let species = Self {
            genomes: vec![head],
            max_fitness,
            avg_fitness,
            stagnancy: 0,
            representative: repr,
            assigned_offspring: 0,
        };

        species
    }

    pub fn can_accomodate(&self, gen: &Genome, sets: &Settings) -> bool {
        let repr_genes = &self.representative.conns;
        let new_genes = &gen.conns;

        let max_repr = repr_genes.iter().max_by_key(|g| g.innov).unwrap();
        let max_new = new_genes.iter().max_by_key(|g| g.innov).unwrap();

        let (mut genes1, mut genes2) = if max_repr.innov > max_new.innov {
            (repr_genes.iter(), new_genes.iter())
        } else {
            (new_genes.iter(), repr_genes.iter())
        };

        let mut p1 = genes1.next();
        let mut p2 = genes2.next();

        let mut disjoint_genes = 0.;
        let mut excess_genes = 0.;
        let mut weight_difference = 0.;
        let mut matching_genes = 0.;
        let mut differing_activs = 0.;

        while p1.is_some() {
            if let (Some(g1), Some(g2)) = (p1, p2) {
                if g1.innov == g2.innov {
                    matching_genes += 1.;
                    weight_difference += (g1.weight - g2.weight).abs();
                    p1 = genes1.next();
                    p2 = genes2.next();
                } else if g2.innov > g1.innov {
                    disjoint_genes += 1.;
                    p1 = genes1.next();
                } else {
                    disjoint_genes += 1.;
                    p2 = genes2.next();
                }

                continue;
            }

            if let (Some(_), None) = (p1, p2) {
                excess_genes += 1.;
                p1 = genes1.next();
            }
        }

        for repr_node in &self.representative.nodes {
            for new_node in &gen.nodes {
                if repr_node.innov == new_node.innov {
                    if repr_node.activation != new_node.activation {
                        differing_activs += 1.;
                    }
                }
            }
        }

        if matching_genes == 0. {
            return false;
        }

        let mut n = if repr_genes.len() > new_genes.len() {
            repr_genes.len() as f64
        } else {
            new_genes.len() as f64
        };

        n = if n < 20. { 1. } else { n };

        let delta = (sets.disjoint_coeff * disjoint_genes / n)
            + (sets.excess_coeff * excess_genes / n)
            + (sets.weight_coeff * weight_difference / matching_genes)
            + (sets.activation_coeff * differing_activs / n);

        delta < sets.speciation_threshold
    }

    pub fn fitness_sharing(&mut self) {
        let len = self.genomes.len() as f64;

        self.genomes
            .iter_mut()
            .for_each(|g| g.fitness = g.fitness / len);

        let total_fitness = self.genomes.iter().fold(0., |acc, g| acc + g.fitness);

        self.avg_fitness = total_fitness / len;
    }

    fn select_parent(&self) -> &Genome {
        let total_fitness = self.genomes.iter().fold(0., |acc, g| acc + g.fitness);

        let mut rng = thread_rng();

        let threshold = rng.gen_range(0., total_fitness);

        let mut current = 0.;

        for genome in &self.genomes {
            current += genome.fitness;
            if current > threshold {
                return genome;
            }
        }

        &self.genomes[0]
    }

    pub fn produce_offspring(&self, amt: usize, sets: &Settings) -> Vec<Genome> {
        let mut rng = thread_rng();

        let mut offspring = Vec::<Genome>::with_capacity(amt);

        for _ in 0..amt {
            if rng.gen::<f64>() < sets.only_mut_rate {
                offspring.push(self.genomes.choose(&mut rng).unwrap().clone())
            } else {
                let parent1 = self.select_parent();
                let parent2 = self.select_parent();

                offspring.push(Genome::crossover(parent1, parent2, sets));
            }
        }

        offspring
    }

    pub fn update_stagnancy(&mut self) {
        if self.genomes.len() == 0 {
            self.stagnancy = u32::MAX;
            return;
        }

        let fitness = self.genomes[0].fitness;

        if fitness <= self.max_fitness {
            self.stagnancy += 1;
        } else {
            self.stagnancy = 0;
            self.max_fitness = fitness;
        }
    }

    pub fn cull_lower_half(&mut self) {
        let len = self.genomes.len();

        if len > 2 {
            self.genomes
                .truncate(if len % 2 == 0 { len / 2 } else { len / 2 + 1 });
        }
    }

    pub fn add_genome(&mut self, gen: Genome) {
        self.genomes.push(gen);
    }

    pub fn sort_genomes(&mut self) {
        self.genomes
            .sort_unstable_by(|a, b| b.fitness.partial_cmp(&a.fitness).unwrap());
    }
}
