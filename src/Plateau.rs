use rand::Rng;
use crate::Piece::piece;

#[derive(Debug, Clone)]
pub struct plateau {
    pieces: Vec<piece>,
    cote: u8,
    max_color: u8,
}

impl plateau {
    pub fn new(cote: u8, max_color: u8) -> plateau {
        plateau {
            pieces: vec![piece::from_max_color(max_color); usize::from(cote * cote)],
            cote,
            max_color,
        }
    }

    pub fn get_cote(&self) -> u8 {
        self.cote
    }

    pub fn get_max_color(&self) -> u8 {
        self.max_color
    }

    pub fn generate(&mut self) {
        //phase de remplissage pour éviter les erreurs oob
        // IMPORTANT: mettre à self.color + 1 pour pouvoir mélanger le tableau.
        self.pieces.fill(piece::from_max_color(self.max_color + 1));

        for _i in 0..self.cote {
            for _j in 0..self.cote {
                let i = _i as i8;
                let j = _j as i8;
                let index = self.to1d(i, j);
                self.pieces[index].n = self.get_face_from_context(i, j - 1, 'N');
                self.pieces[index].s = self.get_face_from_context(i, j + 1, 'S');
                self.pieces[index].e = self.get_face_from_context(i - 1, j, 'E');
                self.pieces[index].w = self.get_face_from_context(i + 1, j, 'W');
            }
        }
    }

    //mode permet d'avoir des swaps
    pub fn shuffle(&mut self, mode : bool) {
        let mut rng = rand::thread_rng();
        let moves = rng.gen_range(5..51);
        let mut rotations = 0;
        let mut swaps = 0;

        for _i in 0..moves {
            let x = rng.gen_range(0..self.cote);
            let y = rng.gen_range(0..self.cote);

            let action = rng.gen_range(0..2);

            if action == 0 {
                self.rotate_at(x, y);
                rotations += 1;
            }
            else if action == 1 && mode == true{
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
                let p : &piece = self.get_at(j, i);
                if self.get_at( j - 1, i).e != p.w {
                    conflicts += 1;
                }
                if self.get_at(j, i - 1).s != p.n {
                    conflicts += 1;
                }
                if self.get_at(j + 1, i).w != p.e {
                    conflicts += 1;
                }
                if self.get_at(j, i + 1).n != p.s {
                    conflicts += 1;
                }
            }
        }

        return conflicts / 2;
    }

    pub fn get_at(&self, x: i8, y: i8) -> &piece {
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
        if y > bound {
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
            out = self.get_at(x, y).s;
        }
        if face == 'S' {
            out = self.get_at(x, y).n;
        }
        if face == 'E' {
            out = self.get_at(x, y).w;
        }
        if face == 'W' {
            out = self.get_at(x, y).e;
        }

        if out == self.max_color + 1 {
            let mut rng = rand::thread_rng();
            out = rng.gen_range(0..(self.max_color + 1));
        }
        println!("{}", out);
        return out;
    }

    pub fn print_tab(&self) {
        for i in 0..self.cote {
            let mut top = String::new();
            let mut mid = String::new();
            let mut bot = String::new();

            for j in 0..self.cote {
                let p : &piece = self.get_at(i as i8, j as i8);
                top += &*format!(" {} ", p.n);
                mid += &*format!("{}#{}", p.w, p.e);
                bot += &*format!(" {} ", p.s);
            }

            println!("{}", top);
            println!("{}", mid);
            println!("{}", bot);
        }
    }
}