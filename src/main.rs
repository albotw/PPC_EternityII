mod Piece;
mod Plateau;

use std::{thread, time};
use rand::Rng;
use crate::Plateau::plateau;

fn main() {
    let mut plateau = plateau::new(5, 3);
    plateau.generate();
    plateau.print_tab();
    plateau.shuffle(false);
    plateau.print_tab();
    let conflicts = plateau.check_conflicts();
    println!("Found {conflicts} conflicts in array");
    RLS(plateau);
}

fn RLS(mut plateau: plateau) {
    let mut i = 0;
    while plateau.check_conflicts() != 0 && i < 1000000 {
        //thread::sleep(time::Duration::from_millis(500));
        i += 1;
        let conflicts = plateau.check_conflicts();
        println!("##### CYCLE {i} #####");
        println!("{conflicts} conflicts remaining");
        let rand_move = get_random_move(&plateau, false);
        println!("[RLS] move generated: {}", rand_move.move_type);
        let mut clone = plateau.clone();
        apply_random_move(&mut clone, rand_move);
        let c_cost = get_cost(&plateau);
        let v_cost = get_cost(&clone);
        if v_cost <= c_cost {
            plateau = clone;
            println!("[RLS] found better configuration");
        }
        else {
            let transition_factor = transition_factor(c_cost, v_cost, 1.0);
            let transition_threshold = transition_threshold();
            println!("[RLS] Transition factor: {transition_factor}");
            println!("[RLS] Transition threshold: {transition_threshold}");
            if transition_threshold <= transition_factor{
                plateau = clone;
                println!("[RLS] transition factor found better configuration");
            }
            else {
                println!("[RLS] no better configuration found");
            }
        }
    }
}

struct rls_move {
    x : u8,
    y : u8,
    x2 : Option<u8>,
    y2: Option<u8>,
    move_type: bool, // true -> rotation, false -> swap
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

fn get_cost(plateau: &plateau) -> i8{
    plateau.check_conflicts() as i8
}

fn transition_factor(c_cost : i8, v_cost : i8, T : f64) -> f64 {
    let a = (c_cost - v_cost) as f64 / T;
    a.exp()
}

fn transition_threshold() -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen()
}

