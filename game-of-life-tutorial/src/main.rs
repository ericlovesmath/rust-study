mod game;

use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{poll, read, Event, KeyCode, KeyEvent},
    execute,
    queue,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
    terminal::{disable_raw_mode, enable_raw_mode},
    Result,
};

use std::io::stdout;
use std::io::Write;
use std::time::Duration;

fn main() -> Result<()> {

    enable_raw_mode()?;

    let mut stdout = stdout();

    // let mut game = game::Universe::new(5, 5);
    // game.set_cells(&[(2, 1), (2, 2), (2, 3)]);

    let mut game = game::Universe::new(10, 10);
    game.set_cells(&[(3, 1), (4, 2), (4, 3), (5, 1), (5, 2)]);

    execute!(
        stdout,
        EnterAlternateScreen,
        // SetForegroundColor(Color::Magenta),
        Hide
    )?;

    loop {

        if poll(Duration::from_millis(200))? {
            if let Event::Key(KeyEvent { code, .. }) = read()? {
                match code { KeyCode::Esc => { break; }
                    _ => {}
                }
            }
        } else {
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
            game.tick();
        }
    }
    execute!(stdout, ResetColor, Show, LeaveAlternateScreen)?;

    disable_raw_mode()?;

    Ok(())
}
