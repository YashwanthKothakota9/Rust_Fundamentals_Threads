use std::time::Instant;

use rayon::prelude::*;

fn is_prime(n: u32) -> bool {
    (2..=n / 2).into_par_iter().all(|i| n % i != 0)
}

fn main() {
    // let numbers: Vec<u64> = (0..1_000_000).collect();
    // let sum = numbers.par_iter().sum::<u64>();
    // println!("{sum}");

    let numbers: Vec<u64> = (2..1_000_000).collect();
    let now = Instant::now();
    let mut primes: Vec<&u64> = numbers
        .par_iter()
        .filter(|&n| is_prime(*n as u32))
        .collect();
    let elapsed = now.elapsed();
    primes.sort();
    println!("{primes:?}");
    println!(
        "It took {} ms to find {} primes",
        elapsed.as_millis(),
        primes.len()
    );
}
