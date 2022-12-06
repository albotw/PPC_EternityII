mod Piece;
mod Plateau;

use std::{thread, time};
use std::cmp::max;
use std::fmt::{Display, Formatter};
use std::time::Instant;
use rand::Rng;
use crate::Plateau::plateau;

fn main() {
    let mut plateau = plateau::new(4, 3);
    plateau.generate();
    //plateau.print_tab();

    plateau.shuffle(true);
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
    while plateau.check_conflicts() != 0{
        //thread::sleep(time::Duration::from_millis(500));
        i += 1;
        let conflicts = plateau.check_conflicts();
        //println!("##### CYCLE {i} #####");
        //println!("{conflicts} conflicts remaining");
        let proximity = plateau.check_proximity();
        let max_proximity = plateau.get_max_proximity();
        //println!("proximity: {proximity} / {max_proximity}");

        let rand_move = get_random_move(&plateau, true);
        //let determined_move = get_move(&plateau, false);
        //println!("[RLS] move generated: {}", rand_move);

        let mut clone = plateau.clone();

        apply_random_move(&mut clone, rand_move);

        let c_cost = get_cost(&plateau);
        let v_cost = get_cost(&clone);
        //println!("c_cost : {c_cost}, v_cost : {v_cost}");
        if v_cost <= c_cost {
            plateau = clone;
            //println!("[RLS] found better configuration");
        }
        else {
            let transition_factor = transition_factor(c_cost, v_cost, temperature);
            let transition_threshold = transition_threshold();

            //println!("[RLS] Transition factor: {transition_factor}");
            //println!("[RLS] Transition threshold: {transition_threshold}");

            if transition_threshold <= transition_factor{
                plateau = clone;
                //println!("[RLS] transition factor found better configuration");
            }
            else {
                //println!("[RLS] no better configuration found");
            }
        }
        if temperature <= 0.001 {
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
    x2 : Option<u8>,
    y2: Option<u8>,
    move_type: bool, // true -> rotation, false -> swap
}

impl Display for rls_move {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "type: {}, x: {}, y: {}", self.move_type, self.x, self.y)
    }
}

//mode permet d'avoir des swaps.
fn get_random_move(plateau : &plateau, mode : bool) -> rls_move {
    let mut rng = rand::thread_rng();
    let move_type = rng.gen_bool(if mode == true {0.5} else {1.0});
    let mut x2 = Option::None;
    let mut y2 = Option::None;
    if move_type == false && mode == true {
        x2 = Option::from(rng.gen_range(0..plateau.get_cote()));
        y2 = Option::from(rng.gen_range(0..plateau.get_cote()));
    }
    rls_move {
        x : rng.gen_range(0..plateau.get_cote()),
        y : rng.gen_range(0..plateau.get_cote()),
        x2,
        y2,
        move_type
    }
}

fn get_move(plateau : &plateau, mode : bool) -> rls_move {
    let mut rng = rand::thread_rng();

    let proximity_factor = plateau.check_proximity() as f64;
    let max_proximity = plateau.get_max_proximity() as f64;
    if proximity_factor <= 0.95 * max_proximity{
        //on priorise les swaps
        return rls_move {
            move_type: false,
            x : rng.gen_range(0..plateau.get_cote()),
            y : rng.gen_range(0..plateau.get_cote()),
            x2 : Option::from(rng.gen_range(0..plateau.get_cote())),
            y2 : Option::from(rng.gen_range(0..plateau.get_cote())),
        }
    }
    else {
        //on priorise les rotations
        return rls_move {
            move_type: true,
            x : rng.gen_range(0..plateau.get_cote()),
            y : rng.gen_range(0..plateau.get_cote()),
            x2: Option::None,
            y2: Option::None
        }
    }


}

fn apply_random_move(plateau: &mut plateau, rls_move : rls_move) {
    if rls_move.move_type == true {
        //appliquer une rotation
        plateau.rotate_at(rls_move.x, rls_move.y);
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

fn get_cost(plateau: &plateau) -> f64{
    let conflicts = plateau.check_conflicts() as f64;
    let proximity = -(plateau.check_proximity() as f64 * 0.1);
    conflicts + proximity
}

fn transition_factor(c_cost : f64, v_cost : f64, T : f64) -> f64 {
    let a = (c_cost - v_cost) / T;
    a.exp()
}

fn transition_threshold() -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen()
}

