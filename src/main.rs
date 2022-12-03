mod Piece;
mod Plateau;

use rand::Rng;
use crate::Piece::piece;
use crate::Plateau::plateau;

fn main() {
    let mut plateau = plateau::new(5, 3);
    plateau.generate();
    plateau.print_tab();
    plateau.shuffle();
    plateau.print_tab();
    let conflicts = plateau.check_conflicts();
    println!("Found {conflicts} conflicts in array");
}


