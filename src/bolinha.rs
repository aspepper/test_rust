/*
 name: bolinha.rs
 compiling: cargo build --release --bin bolinha
*/
use std::io::{stdout, Write};
// Incluir a dependencia: crossterm = "0.29.0"

use std::thread::sleep;
use std::time::Duration;
use crossterm::{
    cursor, event::{self, Event, KeyCode}, terminal, ExecutableCommand,
};

fn main() -> std::io::Result<()> {
    let mut stdout = stdout();
    let (mut x, mut y) = (1, 1);
    let (mut dx, mut dy) = (1, 1);

    // Configura o terminal para modo raw
    stdout.execute(terminal::Clear(terminal::ClearType::All))?;
    stdout.execute(cursor::Hide)?;

    loop {
        // Verifica se há alguma tecla pressionada para sair do loop
        if event::poll(Duration::from_millis(1))? {
            if let Event::Key(key_event) = event::read()? {
                if key_event.code == KeyCode::Char('q') || key_event.code == KeyCode::Esc {
                    break;
                }
            }
        }

        // Apaga a bolinha anterior
        stdout.execute(cursor::MoveTo(x, y))?;
        print!(" ");

        // Atualiza as posições
        x = (x as isize + dx) as u16;
        y = (y as isize + dy) as u16;

        // Desenha a nova bolinha
        stdout.execute(cursor::MoveTo(x, y))?;
        print!("*");

        // Verifica colisão com bordas
        if x == 0 || x == terminal::size()?.0 - 1 {
            dx = -dx;
        }
        if y == 0 || y == terminal::size()?.1 - 1 {
            dy = -dy;
        }

        // Aguarda um pouco para suavizar o movimento
        stdout.flush()?;
        sleep(Duration::from_millis(50));
    }

    // Restaura o terminal ao estado normal
    stdout.execute(cursor::Show)?;
    stdout.execute(terminal::Clear(terminal::ClearType::All))?;
    Ok(())
}