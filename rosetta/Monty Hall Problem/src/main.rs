extern crate rand;

use rand::Rng;

#[derive(PartialEq, Eq, Debug)]
enum Prize {
    Goat,
    Car
}

fn monty_hall_problem(rng: &mut rand::ThreadRng, switch_door: bool) -> bool {
    // Put the prizes behind random doors!
    let mut doors = [Prize::Goat, Prize::Car, Prize::Goat];
    rng.shuffle(&mut doors);

    // Select a doors
    let mut door: usize = rng.gen_range(0, 3);

    // Show another door with a goat behind it
    let mut goat_door = 0;
    for i in 0 .. 3 {
        if i != door && doors[i] == Prize::Goat { goat_door = i; }
    }

    if switch_door {
        for i in 0 .. 3 {
            if i != door && i != goat_door { door = i; break; }
        }
    }

    if doors[door] == Prize::Car { return true } else { return false }
}

fn print_results(results: &Vec<bool>) {
    let mut wins = 0;
    for result in results.iter() {
        if *result { wins += 1; }
    }

    println!("We won the game {} out of {} times\n", wins, results.len())
}

fn main() {
    let mut rng = rand::thread_rng();
    let mut switch_results: Vec<bool> = Vec::new();
    let mut no_switch_results: Vec<bool> = Vec::new();

    for _ in 0 .. 1000 {
        switch_results.push(monty_hall_problem(&mut rng, true));
    }

    for _ in 0 .. 1000 {
        no_switch_results.push(monty_hall_problem(&mut rng, false));
    }

    println!("Switching doors results:");
    print_results(&switch_results);

    println!("Non-switching doors results:");
    print_results(&no_switch_results);
}
