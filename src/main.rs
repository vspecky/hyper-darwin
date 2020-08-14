extern crate neat_rs;

fn main() {
    let sets = neat_rs::Settings::new(2, 1, 150);
    let mut pop = neat_rs::Population::new(sets);

    let mut best_fitness = 0.;

    while best_fitness < 3.5 {
        let genomes = pop.get_citizens();

        for g in genomes {
            let o1 = g.feed_forward(&vec![0., 0.]).unwrap();
            if o1[0] < 0. {
                println!("< 0 o1: {}", o1[0]);
            }
            g.add_fitness(1. - o1[0]);

            let o2 = g.feed_forward(&vec![0., 1.]).unwrap();
            if o2[0] < 0. {
                println!("< 0 o1: {}", o2[0]);
            }
            g.add_fitness(o2[0]);

            let o3 = g.feed_forward(&vec![1., 0.]).unwrap();
            if o3[0] < 0. {
                println!("< 0 o1: {}", o3[0]);
            }
            g.add_fitness(o3[0]);

            let o4 = g.feed_forward(&vec![1., 1.]).unwrap();
            if o4[0] < 0. {
                println!("< 0 o1: {}", o4[0]);
            }
            g.add_fitness(1. - o4[0]);
        }

        pop.next_generation();
        best_fitness = pop.best_fitness;
    }

    let best = &mut pop.best_genome.unwrap();
    println!("{:?}", best);
    println!("Best Fitness: {}", pop.best_fitness);
    println!("Generations: {}", pop.generations);
    println!("0 0: {}", best.feed_forward(&vec![0., 0.]).unwrap()[0]);
    println!("0 1: {}", best.feed_forward(&vec![0., 1.]).unwrap()[0]);
    println!("1 0: {}", best.feed_forward(&vec![1., 0.]).unwrap()[0]);
    println!("1 1: {}", best.feed_forward(&vec![1., 1.]).unwrap()[0]);
}
