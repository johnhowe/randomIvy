extern crate rdrand;
extern crate rand;
use rand::Rng;

fn main() {
    let mut num_ones: u64 = 0;
    let bits = 800_000_000;
    let mut gen = rdrand::RdRand::new().unwrap();
    for _ in 0..bits/64 {
        num_ones += gen.next_u64().count_ones() as u64;
    }
    println!("{} ones - {:1.8}", num_ones, (num_ones as f64) / (bits as f64));
}
