fn main() {
    println!("Hello, world!");
}

struct Piece {
    N : u8,
    S : u8,
    E : u8,
    W : u8,
}

impl Piece {
    pub fn new() -> Piece {
        Piece {
            N: 0,
            S: 0,
            E: 0,
            W: 0,
        }
    }

    pub fn rotate90() {

    }

    pub fn rotate180() {

    }

    pub fn rotate270() {

    }
}