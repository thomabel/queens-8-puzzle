use bracket_lib::random::RandomNumberGenerator;
use ndarray::{Array2, Array1};
use crate::vector::*;

#[derive(Clone, Copy)]
pub struct Queen {
    position: Vector2,
}
impl Queen {
    fn new(position: Vector2) -> Queen {
        Queen { position }
    }
}

#[derive(Clone)]
pub struct Board {
    pieces: Vec<Queen>, 
    dimension: Vector2,
    map: Array2<Option<usize>>,
}
impl Board {
    pub fn new() -> Board {
        let mut pieces = Vec::new();
        let dimension = Vector2::new(8, 8);
        let mut map = Array2::from_elem(dimension.dim(), None);

        // Adds a queen to each column, all on row 0.
        for c in 0..dimension.y as usize {
            pieces.push(Queen::new(Vector2::new(0, c as i32)));
            map[[0, c]] = Some(c);
        }
        Board { pieces, dimension, map }
    }

    pub fn new_random() -> Board {
        let board = Board::new();
        board.randomize()
    }

    /// Randomizes the board layout.
    pub fn randomize(&self) -> Board {
        let mut random_board = self.clone();
        let mut rng = RandomNumberGenerator::new();

        for (j, p) in random_board.pieces.iter_mut().enumerate() {
            let mut i = p.position.index();
            random_board.map[[i.0, i.1]] = None;
            p.position.x = rng.range(0, 8);
            i = p.position.index();
            random_board.map[[i.0, i.1]] = Some(j);
        }

        random_board
    }

    /// Returns the number of pairs of threatened queens.
    /// Does not take into account blocking pieces.
    pub fn check_queens(&self) -> u32 {
        let mut count = 0;

        // Check all queens in sequence
        for (i, queen1) in self.pieces.iter().enumerate() {
            // against each other queen ahead in line.
            for queen2 in self.pieces.iter().skip(i + 1) {
                let diff = queen1.position - queen2.position;
                // Horizontal, Vertical, Diagonal
                if diff.x == 0 || diff.y == 0 || diff.x.abs() == diff.y.abs() {
                    count += 1;
                }
            }
        }

        count
    }

}

impl core::cmp::PartialEq for Board {
    fn eq(&self, other: &Board) -> bool {
        // Make sure they have the same number of pieces.
        if self.pieces.len() != other.pieces.len() {
            return false;
        }

        let mut truth = Array1::<bool>::from_elem(self.pieces.len(), false);
        for queen1 in self.pieces.iter() {
            for (j, queen2) in other.pieces.iter().enumerate() {
                if !truth[j] && queen1.position == queen2.position {
                    truth[j] = true;
                    break;
                }
            }
        }
        for t in truth {
            if !t {
                return false;
            }
        }
        true
    }
}

impl ToString for Board {
    fn to_string(&self) -> String {
        let mut string = String::new();
        for i in self.map.rows() {
            for j in i {
                let ch = 
                match j {
                    Some(_) => 'Q',
                    None => '_',
                };
                string.push(ch);
                string.push(' ');
            }
            string.push('\n');
        }
        string
    }
}
