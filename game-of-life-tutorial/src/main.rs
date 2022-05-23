mod game;

use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{poll, read, Event, KeyCode, KeyEvent},
    execute, queue,
    style::Print,
    terminal::{disable_raw_mode, enable_raw_mode},
    terminal::{Clear, ClearType, DisableLineWrap, EnterAlternateScreen, LeaveAlternateScreen},
    Result,
};

use clap::Parser;

use std::fs::File;
use std::io::{stdout, BufRead, BufReader, Write};
use std::time::Duration;

/// Game of Life simulator written in Rust
#[derive(Parser)]
#[clap(author, version, about)]
struct Args {
    /// Sets the input file to configure initial state of game
    #[clap(short, long)]
    input: Option<String>,

    /// Sets the delay between game ticks. Value is in miliseconds
    #[clap(short, long, default_value_t = 500)]
    delay: u64,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let mut stdout = stdout();

    let mut game = match args.input {
        Some(path) => create_game_from_file(&path),
        None => {
            let mut default_game = game::Universe::new(10, 10);
            default_game.set_cells(&[(3, 1), (4, 2), (4, 3), (5, 1), (5, 2)]);
            default_game
        }
    };

    enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen, DisableLineWrap, Hide)?;

    loop {
        queue!(stdout, Clear(ClearType::All))?;

        let mut i = 0;
        while let Some(line) = game.row_as_string(i) {
            queue!(stdout, MoveTo(0, i as u16), Print(line))?;
            i += 1;
        }

        queue!(
            stdout,
            MoveTo(0, (i + 1) as u16),
            Print("Press Esc to exit...")
        )?;

        stdout.flush()?;

        if poll(Duration::from_millis(args.delay))? {
            if let Event::Key(KeyEvent { code, .. }) = read()? {
                match code {
                    KeyCode::Esc => break,
                    _ => {}
                }
            }
        }

        game.tick();
    }

    execute!(stdout, Show, LeaveAlternateScreen)?;
    disable_raw_mode()?;

    Ok(())
}

fn create_game_from_file(path: &str) -> game::Universe {
    let file = File::open(path).unwrap();
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    let mut rows_num = 0;
    if let Ok(success) = reader.read_line(&mut line) {
        if success > 0 {
            rows_num = line.trim().parse().unwrap();
            line.clear();
        } else {
            panic!("Rows number not detected");
        }
    };
    let mut cols_num = 0;
    if let Ok(success) = reader.read_line(&mut line) {
        if success > 0 {
            cols_num = line.trim().parse().unwrap();
            line.clear();
        } else {
            panic!("Rows number not detected");
        }
    };
    let mut game_universe = game::Universe::new(cols_num, rows_num);
    let mut row = 0;
    let mut live_cells = Vec::<(u32, u32)>::new();
    loop {
        match reader.read_line(&mut line) {
            Ok(0) => break,
            Ok(_) => {
                for (col, char) in line.chars().enumerate() {
                    if char == '1' {
                        live_cells.push((row, col as u32));
                    }
                }
            }
            Err(_) => break,
        }

        line.clear();
        row += 1;
    }
    game_universe.set_cells(&live_cells);
    game_universe
}
