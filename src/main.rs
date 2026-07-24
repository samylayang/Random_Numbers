extern crate rand;
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let n: u32 = rng.gen_range(0..100);
    println!("Random number: {}", n); // Output: Random number: 42
}
