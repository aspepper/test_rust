/*
 name: sieve.rs
 compiling: cargo build --release --bin sieve
*/

use std::time::Instant;
use std::io::{self, Write};

fn main() {
    // Limpa a tela usando sequência ANSI e força o flush do stdout
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap();

    const LIMIT: usize = 100_000_000;
    let mut is_prime = vec![true; LIMIT + 1];
    is_prime[0] = false;
    is_prime[1] = false;

    let start = Instant::now();

    let sqrt = (LIMIT as f64).sqrt() as usize;
    for p in 2..=sqrt {
        if is_prime[p] {
            for multiple in ((p*p)..=LIMIT).step_by(p) {
                is_prime[multiple] = false;
            }
        }
    }

    let ms = start.elapsed().as_millis();
    let count = is_prime.iter().filter(|&&v| v).count();
    println!("Rust Sieve: {} ms – Primes: {}", ms, count);
}