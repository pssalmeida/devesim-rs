extern crate devesim;
extern crate rand;

use devesim::distributions;

fn main() {
    let mut rng = rand::thread_rng();
    println!("X");
    for _ in 0..100000 {
        let x = distributions::weibull(&mut rng, 45.0, 2.0) as u64;
        println!("{}", x);
    }
}

