/*
 name: complex_calc.rs
 compiling: cargo build --release --bin complex_calc
*/

use rayon::prelude::*;
use std::time::Instant;
use std::io::{self, Write};

fn main() {
    // Limpa a tela usando sequência ANSI e força o flush do stdout
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap();

    // Marca o tempo de início
    let start = Instant::now();

    // Realiza cálculos complexos utilizando paralelismo
    let result: f64 = (1..=100_000_000)
        .into_par_iter()
        .map(|i| {
            let i_f64 = i as f64;
            i_f64.sqrt().sin() * i_f64.cos() * i_f64.tan()
        })
        .sum();

    // Calcula a duração em milissegundos
    let duration = start.elapsed();
    println!("Rust: Complex Calc:  {} ms", duration.as_millis());

    // Apenas para utilizar o resultado e evitar otimizações agressivas que removam o loop
    println!("Result: {}", result);
}