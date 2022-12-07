use std::{thread, time};
use std::fmt::{Display, Formatter};
use std::io::{stdout, Write};
use std::time::Instant;
use rand::Rng;

fn main() {
    let mut plateau = plateau::new(3, 3);
    plateau.generate();
    println!("original: ");
    plateau.print_tab();
    plateau.shuffle(true);
    println!("shuffled: ");
    plateau.print_tab();
    let conflicts = plateau.check_conflicts();
    println!("Found {conflicts} conflicts in array");
    let proximity = plateau.check_proximity();
    println!("Proximity factor: {proximity}");
    RLS(plateau);
}

fn RLS(mut plateau: plateau) {
    let mut i = 0;
    let mut temperature = 100_000.0;
    let start = Instant::now();
    //verrouille la sortie standard pour ne pas ralentir la RLS à cause des println
    //https://doc.rust-lang.org/std/macro.println.html
    let mut lock = stdout().lock();
    while plateau.check_conflicts() != 0{
        //thread::sleep(time::Duration::from_millis(500));
        i += 1;
        let conflicts = plateau.check_conflicts();
        writeln!(lock, "##### CYCLE {i} #####").unwrap();
        writeln!(lock, "{conflicts} conflicts remaining").unwrap();
        let rand_move = get_random_move(&plateau);
        let c_cost = get_move_cost(&plateau, &rand_move);
        if c_cost != 0.0 {
            apply_move(&mut plateau, &rand_move);
            let v_cost = get_move_cost(&plateau, &rand_move);
            writeln!(lock, "c_cost: {c_cost}, v_cost: {v_cost}").unwrap();

            //mouvement appliqué sur le plateau courant par défaut.
            if v_cost > c_cost {
                let transition_factor = transition_factor(c_cost, v_cost, temperature);
                let transition_threshold = transition_threshold();
                writeln!(lock, "threshold: {transition_threshold} / factor: {transition_factor}").unwrap();
                if transition_threshold > transition_factor {
                    rollback_move(&mut plateau, &rand_move);
                    writeln!(lock, "move rolled back").unwrap();
                }
            }
        }

        if temperature <= 0.001 {
            writeln!(lock, "temperature reset").unwrap();
            temperature = 1000.0;
        }
        else {
            temperature *= 0.999;
        }
    }
    let conflicts = plateau.check_conflicts();
    let proximity = plateau.check_proximity();
    let elapsed = start.elapsed();
    println!("[RLS] final temperature: {temperature}");
    println!("[RLS] final proximity : {proximity}");
    println!("[RLS] end. {conflicts} remaining after {i} cycles. {} s elapsed", elapsed.as_secs());
}

//mouvement qui peut être appliqué sur le plateau
struct rls_move {
    x : u8,
    y : u8,
    x2 : Option<u8>, //valeur optionnelle (swap uniquement)
    y2: Option<u8>, // valeur optionnelle (swap uniquement)
    move_type: bool, // true -> rotation, false -> swap
    rotations_count : u8 // de 1 à 3 rotations par mouvement
}

//permet d'afficher la structure telle qu'elle
impl Display for rls_move {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "type: {}, x: {}, y: {}", self.move_type, self.x, self.y)
    }
}

//retourne un mouvement généré aléatoirement
fn get_random_move(plateau : &plateau) -> rls_move {
    let mut rng = rand::thread_rng();
    let move_type = rng.gen_bool(0.5);
    let mut x2 = Option::None;
    let mut y2 = Option::None;
    if move_type == false {
        x2 = Option::from(rng.gen_range(0..plateau.get_cote()));
        y2 = Option::from(rng.gen_range(0..plateau.get_cote()));
    }
    rls_move {
        x : rng.gen_range(0..plateau.get_cote()),
        y : rng.gen_range(0..plateau.get_cote()),
        rotations_count : rng.gen_range(1..=3),
        x2,
        y2,
        move_type
    }
}

//applique un mouvement sur un plateau
fn apply_move(plateau: &mut plateau, rls_move : &rls_move) {
    if rls_move.move_type == true {
        //appliquer une rotation (de 90° à 270°)
        for i in 0..rls_move.rotations_count {
            plateau.rotate_at(rls_move.x, rls_move.y);
        }
    }
    else if rls_move.move_type == false {
        //appliquer un swap
        //vérification du remplissage des champs optionnels
        if let Some(x2) = rls_move.x2 {
            if let Some(y2) = rls_move.y2 {
                plateau.swap(rls_move.x, rls_move.y, x2, y2);
            }
        }
    }
}

//annuler un mouvement
fn rollback_move(plateau : &mut plateau, rls_move : &rls_move) {
    if rls_move.move_type == true {
        let rotations =  4 - rls_move.rotations_count;
        for i in 0..rotations {
            plateau.rotate_at(rls_move.x, rls_move.y);
        }
    }
    else if rls_move.move_type == false {
        if let Some(x2) = rls_move.x2 {
            if let Some(y2) = rls_move.y2 {
                plateau.swap(x2, y2, rls_move.x, rls_move.y);
            }
        }
    }
}

//pour récupérer le cout a l'échelle du plateau
fn get_cost(plateau: &plateau) -> f64{
    let conflicts = plateau.check_conflicts() as f64;
    let proximity = -(plateau.check_proximity() as f64 * 0.1);
    return conflicts + proximity
}

//récupère le cout d'un mouvement
fn get_move_cost(plateau : &plateau, rls_move : &rls_move) -> f64 {
    let conflicts_p1 = plateau.check_conflicts_for_piece(rls_move.x as i8, rls_move.y as i8) as f64;
    if rls_move.move_type == false {
        let x2 = rls_move.x2.unwrap();
        let y2 = rls_move.y2.unwrap();
        let conflicts_p2 = plateau.check_conflicts_for_piece(x2 as i8, y2 as i8) as f64;
        return conflicts_p1 + conflicts_p2;
    }
    else {
        return conflicts_p1;
    }

}

//critère de métropolis
fn transition_factor(c_cost : f64, v_cost : f64, T : f64) -> f64 {
    let a = (c_cost - v_cost) / T;
    return a.exp();
}

//retourne un entier aléatoire entre 0 et 1
fn transition_threshold() -> f64 {
    let mut rng = rand::thread_rng();
    return rng.gen()
}
//==================================================================================================
#[derive(Debug, Clone, Copy)] // pour pouvoir cloner le tableau
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

//==================================================================================================
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
        //phase de remplissage pour éviter les erreurs out of bounds
        // IMPORTANT: mettre à self.color + 1 pour pouvoir mélanger le tableau.
        self.pieces.fill(piece::from_max_color(self.max_color + 1));

        for _i in 0..self.cote {
            for _j in 0..self.cote {
                let i = _i as i8;
                let j = _j as i8;
                let index = self.to1d(j, i);
                self.pieces[index].n = self.get_face_from_context(j, i - 1, 'N');
                self.pieces[index].s = self.get_face_from_context(j, i + 1, 'S');
                self.pieces[index].e = self.get_face_from_context(j + 1, i, 'E');
                self.pieces[index].w = self.get_face_from_context(j - 1, i, 'W');
            }
        }
    }

    //mode permet d'avoir des swaps
    //mélanger le tableau
    pub fn shuffle(&mut self, mode : bool) {
        let mut rng = rand::thread_rng();
        let moves = rng.gen_range(5..=50);
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

        println!("Shuffle: {moves} moves, {rotations} rotations, {swaps} swaps");
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
                conflicts += self.check_conflicts_for_piece(j, i);
            }
        }

        return conflicts / 2;
    }

    pub fn check_conflicts_for_piece(&self, x : i8, y : i8) -> u8 {
        let mut conflicts = 0;
        let p : &piece = self.get_at(x, y);
        if self.get_at( x - 1, y).e != p.w {
            conflicts += 1;
        }
        if self.get_at(x, y - 1).s != p.n {
            conflicts += 1;
        }
        if self.get_at(x + 1, y).w != p.e {
            conflicts += 1;
        }
        if self.get_at(x, y + 1).n != p.s {
            conflicts += 1;
        }

        return conflicts;
    }

    pub fn get_max_proximity(&self) -> u8 {
        (self.cote * self.cote * 4) / 2
    }

    pub fn check_proximity(&self) -> u8 {
        let mut proximity = 0;
        for _i in 0..self.cote {
            for _j in 0..self.cote {
                let i = _i as i8;
                let j = _j as i8;
                proximity += self.check_proximity_for_piece(j, i);
            }
        }

        return proximity / 2;
    }

    pub fn check_proximity_for_piece(&self, x : i8, y : i8) -> u8 {
        let mut proximity = 0;
        let current_piece : &piece = self.get_at(x, y);

        let north_neighbor : &piece = self.get_at(x, y - 1);
        let west_neighbor : &piece = self.get_at(x - 1, y);
        let east_neighbor : &piece = self.get_at(x + 1, y);
        let south_neighbor : &piece = self.get_at(x, y + 1);

        if north_neighbor.n == current_piece.n
            || north_neighbor.e == current_piece.n
            || north_neighbor.s == current_piece.n
            || north_neighbor.w == current_piece.n {
            proximity += 1;
        }
        if west_neighbor.n == current_piece.w
            || west_neighbor.e == current_piece.w
            || west_neighbor.s == current_piece.w
            || west_neighbor.w == current_piece.w {
            proximity += 1;
        }
        if east_neighbor.n == current_piece.e
            || east_neighbor.s == current_piece.e
            || east_neighbor.e == current_piece.e
            || east_neighbor.w == current_piece.e {
            proximity += 1;
        }

        if south_neighbor.n == current_piece.s
            || south_neighbor.s == current_piece.s
            || south_neighbor.e == current_piece.s
            || south_neighbor.w == current_piece.s {
            proximity += 1;
        }

        return proximity;
    }

    pub fn get_at(&self, x: i8, y: i8) -> &piece {
        let u8_position = self.to1d(x, y);
        return &self.pieces[usize::from(u8_position)];
    }

    pub fn to1d(&self, mut x : i8, mut y : i8) -> usize {
        let bound = (self.cote -1 ) as i8;
        if x < 0 {
            x = bound;
        }
        else if x > bound {
            x = 0;
        }
        if y < 0 {
            y = bound;
        }
        else if y > bound {
            y = 0;
        }

        let u8_position : u8 =  (x + (self.cote as i8 * y)) as u8;
        //println!("x: {x}, y: {y}, to1d: {u8_position}");
        return usize::from(u8_position);
    }

    pub fn rotate_at(&mut self, x: u8, y: u8) {
        let position = self.to1d(x as i8, y as i8);
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

        //si c'est la couleur par défaut (car pas de null en rust)
        //on en tire une au hasard
        if out == self.max_color + 1 {
            let mut rng = rand::thread_rng();
            out = rng.gen_range(0..=self.max_color);
        }
        //println!("{}", out);
        return out;
    }

    pub fn print_tab(&self) {
        for i in 0..self.cote {
            let mut top = String::new();
            let mut mid = String::new();
            let mut bot = String::new();

            for j in 0..self.cote {
                let p : &piece = self.get_at(j as i8, i as i8);
                top += &*format!(" {} ", p.n);
                mid += &*format!("{}#{}", p.w, p.e);
                bot += &*format!(" {} ", p.s);
            }

            println!("{}", top);
            println!("{}", mid);
            println!("{}", bot);
        }
        println!();
    }
}