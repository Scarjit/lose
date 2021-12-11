const NAMES : [&str; 5] = ["Ferdinand", "Steffen", "Patrick", "Lukas", "Leon"];

use std::thread;
use std::time::Duration;
use rand::thread_rng;
use rand::seq::SliceRandom;

fn main() {
    let mut from_names = NAMES.clone();
    let mut to_names = NAMES.clone();

    from_names.shuffle(&mut thread_rng());
    to_names.shuffle(&mut thread_rng());

    loop {
        let mut xmatch = false;
        for i in 0..from_names.len() {
            if from_names[i] == to_names[i] {
                xmatch = true;
            }
        }
        if xmatch == false {
            break
        }
        from_names.shuffle(&mut thread_rng());
        to_names.shuffle(&mut thread_rng());
    }

    for i in 0..from_names.len() {
        println!("{} -> {}", from_names[i], to_names[i]);
        thread::sleep(Duration::from_secs(69));
    }
}
