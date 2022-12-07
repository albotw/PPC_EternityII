mod Piece;
mod Plateau;

use std::{thread, time};
use std::fmt::{Display, Formatter};
use std::io::{stdout, Write};
use std::time::Instant;
use rand::Rng;
use crate::Plateau::plateau;

fn main() {
    let mut plateau = plateau::new(4, 3);
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

struct rls_move {
    x : u8,
    y : u8,
    x2 : Option<u8>, //valeur optionnelle (swap uniquement)
    y2: Option<u8>, // valeur optionnelle (swap uniquement)
    move_type: bool, // true -> rotation, false -> swap
    rotations_count : u8 // de 1 à 3 rotations par mouvement
}

//permet d'afficher rls_move dans les macros println
impl Display for rls_move {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "type: {}, x: {}, y: {}", self.move_type, self.x, self.y)
    }
}

//retourne un mouvement aléatoire
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

//appliquer un mouvement sur le plateau
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
        let rotations =  4 - rls_move.rotations_count; //
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
    //return conflicts as f64;
}

fn get_move_cost(plateau : &plateau, rls_move : &rls_move) -> f64 {
    let conflicts_p1 = plateau.check_conflicts_for_piece(rls_move.x as i8, rls_move.y as i8) as f64;
    if rls_move.move_type == false {
        let x2 = rls_move.x2.unwrap();
        let y2 = rls_move.y2.unwrap();
        let conflicts_p2 = plateau.check_conflicts_for_piece(x2 as i8, y2 as i8) as f64;
        return conflicts_p1 + conflicts_p2;
    }
    else {
        //let mut proximity = plateau.check_proximity_for_piece(rls_move.x as i8, rls_move.y as i8) as f64;
        //proximity = -(proximity * 0.1);
        //return conflicts + proximity;
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

