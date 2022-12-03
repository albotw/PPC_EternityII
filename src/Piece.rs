#[derive(Debug, Clone, Copy)]
pub struct piece {
    pub n: u8,
    pub s: u8,
    pub e: u8,
    pub w: u8,
}

impl piece {
    pub fn new() -> piece {
        piece {
            n: 0,
            s: 0,
            e: 0,
            w: 0,
        }
    }

    pub fn from_max_color(max_color: u8) -> piece {
        piece {
            n: max_color,
            s: max_color,
            e: max_color,
            w: max_color
        }
    }

    pub fn rotate90(&mut self) {
        let tmp = self.n;
        self.n = self.e;
        self.e = self.s;
        self.s = self.w;
        self.w = tmp;
    }
}