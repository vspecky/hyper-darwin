use crate::genome::Genome;
use crate::history::History;
use crate::settings::Settings;
use crate::species::Species;

use std::vec::Vec;

pub struct Population {
    sets: Settings,
    population: Vec<Genome>,
    species: Vec<Species>,
    hist: History,
    pub best_fitness: f64,
    pub best_genome: Option<Genome>,
    next_species_id: u32,
    pub generations: u64,
}

impl Population {
    pub fn new(sets: Settings) -> Self {
        let inputs = sets.inputs;
        let outputs = sets.outputs;
        let pop_size = sets.pop_size as usize;
        let mut pop = Self {
            sets: sets,
            population: Vec::<Genome>::with_capacity(pop_size),
            species: Vec::new(),
            hist: History::new(inputs, outputs),
            best_fitness: 0.,
            best_genome: None,
            next_species_id: 1,
            generations: 0,
        };

        pop.reset();

        pop
    }

    fn reset(&mut self) {
        self.population.clear();
        self.species.clear();
        self.best_fitness = 0.;
        self.best_genome = None;
        self.next_species_id = 1;
        self.generations = 0;
        self.hist = History::new(self.sets.inputs, self.sets.outputs);

        for _ in 0..self.sets.pop_size {
            let genome = Genome::new(self.sets.inputs, self.sets.outputs, false);
            self.population.push(genome);
        }
    }

    pub fn next_generation(&mut self) {
        self.population
            .sort_unstable_by(|a, b| b.fitness.partial_cmp(&a.fitness).unwrap());

        let mut this_champ = self.population[0].clone();
        if this_champ.fitness > self.best_fitness {
            self.best_fitness = this_champ.fitness;
            self.best_genome = Some(this_champ.clone());
        }

        this_champ.fitness = 0.;

        self.speciate_population();

        self.species.iter_mut().for_each(|s| {
            s.sort_genomes();
            s.update_stagnancy();
            s.fitness_sharing();
            s.cull_lower_half();
        });

        let allowed_stagnancy = self.sets.allowed_stagnancy;

        self.species.retain(|s| s.stagnancy < allowed_stagnancy);

        let total_avg_fitness = self.species.iter().fold(0., |acc, s| acc + s.avg_fitness);

        let pop_size = self.sets.pop_size;

        self.species.iter_mut().for_each(|s| {
            s.assigned_offspring = (s.avg_fitness / total_avg_fitness * pop_size as f64) as usize;
        });

        self.species.retain(|s| s.assigned_offspring > 0);

        let mut progeny = Vec::<Genome>::with_capacity(pop_size as usize);

        for species in &self.species {
            let mut new_offspring = species.assigned_offspring;

            if species.genomes.len() > 3 {
                let mut champ = species.genomes[0].clone();
                champ.fitness = 0.;
                progeny.push(champ);
                new_offspring -= 1;
            }

            for mut child in species.produce_offspring(new_offspring, &self.sets) {
                child.mutate(&mut self.hist, &self.sets);
                child.fitness = 0.;
                progeny.push(child);
            }
        }

        if progeny.len() < pop_size as usize {
            while progeny.len() < pop_size as usize {
                let mut another_child = this_champ.clone();
                another_child.mutate(&mut self.hist, &self.sets);
                progeny.push(another_child);
            }
        }

        self.population = progeny;
        self.generations += 1;
    }

    pub fn get_citizens(&mut self) -> &mut Vec<Genome> {
        &mut self.population
    }

    fn speciate_population(&mut self) {
        for species in &mut self.species {
            species.genomes.clear();
        }

        'outer: for genome in &self.population {
            for species in &mut self.species {
                if species.can_accomodate(genome, &self.sets) {
                    species.add_genome(genome.clone());
                    continue 'outer;
                }
            }

            let new_spec = Species::new(genome.clone(), self.next_species_id);
            self.next_species_id += 1;
            self.species.push(new_spec);
        }

        self.population.clear();
    }
}
