
#[test]
fn conflicts() {
    use crate::puzzle::Board;
    let puzzle = Board::new();
    let conflict = puzzle.check_queens();
    assert_eq!(conflict, 28);
}