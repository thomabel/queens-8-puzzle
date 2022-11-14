use bracket_lib::random::RandomNumberGenerator;
use ndarray::{prelude::*, concatenate};
use crate::vector::Vector2;

#[derive(Clone)]
pub struct Gene (Array1<u8>);

impl Gene {
    /// Creates a new gene with all queens on row 0.
    pub fn new(dimension: &Vector2) -> Gene {
        let dim = ndarray::Dim(dimension.y as usize);
        let array = Array1::<u8>::zeros(dim);
        Gene(array)
    }

    /// Randomly changes the row of every queen.
    pub fn randomize(&mut self, dimension: &Vector2, rng: &mut RandomNumberGenerator) {
        for i in 0..dimension.x as usize {
            self.0[i] = rng.range(0, dimension.x as u8);
        }
    }

    /// Counts how many queens are attacking.
    pub fn fitness(&self) -> u32 {
        let mut count = 0;
        // Check all queens in sequence
        for (i, queen1) in self.0.iter().enumerate() {
            // against each other queen ahead in line.
            for (j, queen2) in self.0.iter().skip(i+1).enumerate() {
                let x = i as i32 - j as i32; 
                let y = *queen1 as i32 - *queen2 as i32;
                // Horizontal, Vertical, Diagonal
                if x == 0 || y == 0 || x == y {
                    count += 1;
                }
            }
        }
        28 - count
    }

    /// Chooses a random queen and changes it to a random row.
    pub fn mutate(&mut self, max: u8, rng: &mut RandomNumberGenerator) {
        let index = rng.range(0, self.0.len());
        self.0[index] = rng.range(0, max);
    }

    /// Chooses a random point in the center of the genes and combines into a new gene.
    pub fn crossover(&self, other: &Gene, rng: &mut RandomNumberGenerator) -> (Gene, Gene) {
        // Determine ranges
        let length = self.0.len();
        let index = rng.range(2, length - 2);
        let left = 0..index;
        let right = index..length;

        // Get slices
        let l1 = self.0.slice(s![left.clone()]);
        let l2 = other.0.slice(s![left]);
        let r1 = other.0.slice(s![right.clone()]);
        let r2 = self.0.slice(s![right]);

        // Concat
        (Gene(concatenate(Axis(0), &[l1, r1]).unwrap()),
        Gene(concatenate(Axis(0), &[l2, r2]).unwrap()))
    }

}

impl ToString for Gene {
    fn to_string(&self) -> String {
        let mut string = String::new();
        for i in self.0.iter() {
            string += &i.to_string();
            string.push(' ');
        }
        string
    }
}

impl Gene {
    pub fn to_board(&self) -> String {
        let mut board = String::new();
        // for each row
        for i in 0..8 {
            // check each piece to see if it's in that row
            for q in self.0.iter() {
                if *q == i {
                    board.push_str("-Q-");
                }
                else {
                    board.push_str("|_|");
                }
            }
            board.push('\n');
        }
        board
    }
}