/*
 name: matrix_mul.rs
 compiling: cargo build --release --bin matrix_mul
*/

use rayon::prelude::*;
use std::time::Instant;
use std::io::{self, Write};

fn main() {
    // Limpa a tela usando sequência ANSI e força o flush do stdout
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap();

    const N: usize = 1000;
    // Inicializa matrizes
    let a = vec![vec![1.0f64; N]; N];
    let b = vec![vec![2.0f64; N]; N];
    let mut c = vec![vec![0.0f64; N]; N];

    let start = Instant::now();

    // Paraleliza no nível de linhas
    c.par_iter_mut().enumerate().for_each(|(i, row)| {
        for j in 0..N {
            let mut sum = 0.0;
            for k in 0..N {
                sum += a[i][k] * b[k][j];
            }
            row[j] = sum;
        }
    });

    let ms = start.elapsed().as_millis();
    println!("Rust Matrix Mul: {} ms", ms);
    println!("c[0][0] = {}", c[0][0]);
}