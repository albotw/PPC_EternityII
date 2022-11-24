use rand::Rng;

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
    maxColor: u8,
}

impl Plateau {
    pub fn new(cote : u8, maxColor: u8) -> Plateau {
        Plateau {
            pieces : Vec::new(),
            cote: cote,
            maxColor: maxColor
        }
    }

    pub fn generate(&self) {
        for i in 0..self.cote {
            for j in 0..self.cote {
                let p  = Piece {
                    N : self.get_face_from_context(i, j - 1, 'N'),
                    S : self.get_face_from_context(i, j + 1, 'S'),
                    E : self.get_face_from_context(i - 1, j, 'E'),
                    W : self.get_face_from_context(i + 1, j, 'W'),
                };
            }
        }
    }

    pub fn swap(&mut self, x1 : u8, y1 : u8, x2 : u8, y2 : u8) {

    }

    pub fn get_at(&self, mut x : u8, mut y : u8) -> &Piece{
        if (x < 0) {
            x = self.cote - 1;
        }
        if (x > self.cote - 1) {
            x = 0;
        }
        if (y < 0) {
            y = self.cote - 1;
        }
        if (y > self.cote - 1) {
            y = 0;
        }

        return &self.pieces[usize::from(x + (self.cote * y))];
    }

    pub fn set_at(&mut self, x : u8, y : u8, p : Piece) {
        self.pieces.swap_remove()
    }

    fn get_face_from_context(&self, x: u8, y : u8, face : char) -> u8 {
        let mut out = self.maxColor + 1;
        if (face == 'N') {
            out = self.get_at(x, y).S;
        }
        if (face == 'S') {
            out = self.get_at(x, y).N;
        }
        if (face == 'E') {
            out = self.get_at(x, y).W;
        }
        if (face == 'W') {
            out = self.get_at(x, y).E;
        }

        if (out == self.maxColor + 1) {
            let mut rng = rand::thread_rng();
            out = rng.gen_range(0, (self.maxColor + 1));
        }

        return out;
    }
}