/*
Thomas Abel
AI
2022-10-27
*/

use puzzle::Board;

mod puzzle;
mod vector;
mod tests;

fn main() {
    experiment();
}

fn experiment() {
    let board = Board::new_random();
    println!("{}", board.to_string());
    println!("Conflicts: {}", board.check_queens());
}
