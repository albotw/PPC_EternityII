use std::fmt::format;
use color_print::cformat;
use color_print::cstr;
use rand::Rng;

fn main() {
    let mut plateau : Plateau = Plateau::new(5, 3);
    plateau.generate();
    plateau.shuffle();
    let conflicts = plateau.check_conflicts();
    println!("Found {conflicts} conflicts in array");
}

#[derive(Debug, Clone, Copy)]
struct Piece {
    N: u8,
    S: u8,
    E: u8,
    W: u8,
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

    pub fn from_max_color(max_color: u8) -> Piece {
        Piece{
            N: max_color,
            S: max_color,
            E: max_color,
            W: max_color
        }
    }

    pub fn rotate90(&mut self) {
        let tmp = self.N;
        self.N = self.E;
        self.E = self.S;
        self.S = self.W;
        self.W = tmp;
    }
}

struct Plateau {
    pieces: Vec<Piece>,
    cote: u8,
    max_color: u8,
}

impl Plateau {
    pub fn new(cote: u8, max_color: u8) -> Plateau {
        Plateau {
            pieces: vec![Piece::from_max_color(max_color); usize::from(cote * cote)],
            cote,
            max_color,
        }
    }

    pub fn generate(&mut self) {
        //phase de remplissage pour éviter les erreurs oob
        let capa = self.pieces.capacity().to_string();
        println!("{capa}");

        self.pieces.fill(Piece::from_max_color(self.max_color));

        let size = self.pieces.len().to_string();
        println!("{size}");

        for _i in 0..self.cote {
            for _j in 0..self.cote {
                let i = _i as i8;
                let j = _j as i8;
                let index = self.to1d(i, j);
                self.pieces[index].N = self.get_face_from_context(i, j - 1, 'N');
                self.pieces[index].S = self.get_face_from_context(i, j + 1, 'S');
                self.pieces[index].E = self.get_face_from_context(i - 1, j, 'E');
                self.pieces[index].W = self.get_face_from_context(i + 1, j, 'W');
            }
        }
    }

    pub fn shuffle(&mut self) {
        let mut rng = rand::thread_rng();
        let moves = rng.gen_range(5..51);
        let mut rotations = 0;
        let mut swaps = 0;

        for i in 0..moves {
            let x = rng.gen_range(0..self.cote);
            let y = rng.gen_range(0..self.cote);

            let action = rng.gen_range(0..2);

            if action == 0 {
                self.rotate_at(x, y);
                rotations += 1;
            }
            else if action == 1 {
                let x2 = rng.gen_range(0..self.cote);
                let y2 = rng.gen_range(0..self.cote);

                self.swap(x, y, x2, y2);
                swaps += 1;
            }
        }

        println!("Mélange: {moves} mouvements, {rotations} rotations, {swaps} échanges")
    }

    pub fn swap(&mut self, x1: u8, y1: u8, x2: u8, y2: u8) {
        let index_piece_1 = x1 + (self.cote * y1);
        let index_piece_2 = x2 + (self.cote * y2);
        self.pieces
            .swap(usize::from(index_piece_1), usize::from(index_piece_2));
    }

    pub fn check_conflicts(&self) -> u8 {
        let mut conflicts = 0;
        for _i in 0..self.cote{
            for _j in 0..self.cote {
                let i = _i as i8;
                let j = _j as i8;
                let p : &Piece = self.get_at(j, i);
                if self.get_at( j - 1, i).E != p.W {
                    conflicts += 1;
                }
                if self.get_at(j, i - 1).S != p.N {
                    conflicts += 1;
                }
                if self.get_at(j + 1, i).W != p.E {
                    conflicts += 1;
                }
                if self.get_at(j, i + 1).N != p.S {
                    conflicts += 1;
                }
            }
        }

        return conflicts / 2;
    }

    pub fn get_at(&self, mut x: i8, mut y: i8) -> &Piece {
        let u8_position = self.to1d(x, y);
        return &self.pieces[usize::from(u8_position)];
    }

    pub fn to1d(&self, mut x : i8, mut y : i8) -> usize {
        let bound = (self.cote - 1) as i8;
        if x < 0 {
            x = bound;
        }
        if x > bound {
            x = 0;
        }
        if y < 0 {
            y = bound;
        }
        if (y > bound) {
            y = 0;
        }

        let u8_position : u8 =  (x + (bound * y)) as u8;
        return usize::from(u8_position);
    }

    pub fn rotate_at(&mut self, x: u8, y: u8) {
        let position = usize::from(x + (self.cote * y));
        self.pieces[position].rotate90();
    }

    fn get_face_from_context(&self, x: i8, y: i8, face: char) -> u8 {
        let mut out = self.max_color + 1;
        if face == 'N' {
            out = self.get_at(x, y).S;
        }
        if face == 'S' {
            out = self.get_at(x, y).N;
        }
        if face == 'E' {
            out = self.get_at(x, y).W;
        }
        if face == 'W' {
            out = self.get_at(x, y).E;
        }

        if out == self.max_color + 1 {
            let mut rng = rand::thread_rng();
            out = rng.gen_range(0..(self.max_color + 1));
        }

        return out;
    }

    fn format_face(color: u8) {
        let color_table = ["red", "green", "yellow", "blue", "magenta", "cyan", "white", "bright-red", "bright-green", "bright-yellow"];
        let index = usize::from(color);
        let string = format!("<{}>{}</{}>", color_table[index], color, color_table[index]);
        let out = cstr!(string);
        return out;
    }
}
