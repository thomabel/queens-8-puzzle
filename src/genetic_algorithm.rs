use bracket_lib::random::RandomNumberGenerator;
use ndarray::Array1;
use crate::{gene::Gene, vector::Vector2};

pub struct GeneticAlgorithm {
    population:     Array1<Gene>,
    fitness:        Array1<u32>,
    probability:    Array1<f32>,
    board_size:     Vector2,
}

impl GeneticAlgorithm {
    pub fn new(board_size: Vector2, population_size: usize) -> GeneticAlgorithm {
        let gene = Gene::new(&board_size);
        let mut population = Array1::from_elem(population_size, gene);
        let mut rng = RandomNumberGenerator::new();
        for g in population.iter_mut() {
            g.randomize(&board_size, &mut rng)
        }
        let fitness = Array1::zeros(population_size);
        let probability = Array1::zeros(population_size);
        GeneticAlgorithm { population, fitness, probability, board_size }
    }

    pub fn run(&mut self, num_iterations: usize, mutate_pct: f32) -> Result {
        let mut rng = RandomNumberGenerator::new();
        let mut correct = Vec::new();
        let mut avg_fitness = Vec::new();
        let mut sample = Vec::new();
        // Initial population.
        self.fitness();
        self.probability();

        for _iteration in 0..num_iterations {
            // Get all parents & create children
            self.reproduce(&mut rng, mutate_pct);
            // Determine fitness
            self.fitness();
            // and probability
            self.probability();

            // Check population for winning boards.
            for (i, value) in self.fitness.iter().enumerate() {
                if *value == 28 {
                    correct.push(self.population[i].clone());
                }
            }

            // Average fitness
            avg_fitness.push(self.average_fitness());
            // Sample
            if _iteration % 1000 == 0 {
                sample.push(self.population[0].clone());
            }
        }

        Result{ correct, avg_fitness, sample }
    }

    /// Creates a new population using the current population.
    fn reproduce(&mut self, rng: &mut RandomNumberGenerator, mutate_pct: f32) {
        let popsize = self.population.len();
        let mut children = Vec::new();
        
        for _i in 0..popsize/2 {
            // Get first parent. Any one will do.
            let p = rng.range(0., 1.);
            let parent1 = self.choose_parent(p);
            
            // Second parent must be different.
            let mut parent2 = parent1;
            while parent1 == parent2 {
                let p = rng.range(0., 1.);
                parent2 = self.choose_parent(p);
            }

            // Create the two children from crossover.
            let mut cross = self.population[parent1].crossover(&self.population[parent2], rng);

            // Mutate based on probability.
            let max = self.board_size.x as u8;
            let m0 = rng.range(0., 1.);
            let m1 = rng.range(0., 1.);
            if m0 < mutate_pct {
                cross.0.mutate(max, rng);
            }
            if m1 < mutate_pct {
                cross.1.mutate(max, rng);
            }

            // Push them to the vector.
            children.push(cross.0);
            children.push(cross.1);
        }

        self.population = Array1::from_vec(children);
    }
    
    /// Chooses a parent when making children.
    fn choose_parent(&self, p: f32) -> usize {
        let mut cumulative_probability = 0.0;
        for (i, prob) in self.probability.iter().enumerate() {
            cumulative_probability += prob;
            if p <= cumulative_probability {
                return i;
            }
        }
        0
    }

    /// Calculates the fitness value for everyone in the population.
    fn fitness(&mut self) {
        for (g, gene) in self.population.iter().enumerate() {
            self.fitness[g] = gene.fitness();
        }
    }

    /// Calculates the probability value for everyone in the population.
    fn probability(&mut self) {
        let sum = self.fitness.sum() as f32;
        for (f, fitness) in self.fitness.iter().enumerate() {
            self.probability[f] = *fitness as f32 / sum;
        }
    }

    /// Returns the average value of fitness among the current population.
    pub fn average_fitness(&self) -> f32 {
        let sum = self.fitness.sum() as f32;
        let total = self.fitness.len() as f32;
        sum / total
    }

}

pub struct Result {
    pub correct: Vec<Gene>,
    pub avg_fitness: Vec<f32>,
    pub sample: Vec<Gene>,
}
