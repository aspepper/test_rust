/*
cargo build --release --bin thread_spawn
*/

use std::thread;
use std::time::Instant;
use std::io::{self, Write};

fn trivial_task() {
    let mut _x = 0;
    _x += 1;
}

fn main() {
    // Limpa a tela usando sequência ANSI e força o flush do stdout
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap();

    const N: usize = 1000;
    let mut handles = Vec::with_capacity(N);

    let start = Instant::now();

    for _ in 0..N {
        handles.push(thread::spawn(trivial_task));
    }
    for h in handles {
        let _ = h.join();
    }

    let ms = start.elapsed().as_millis();
    println!("Rust Thread spawn ({}): {} ms", N, ms);
}