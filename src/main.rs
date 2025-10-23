mod utils;
use utils::{
    Result,
    graphics::{Config, render, update},
};

use ratatui::prelude::{Terminal, CrosstermBackend};
use crossterm::{
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode}
};

fn main() -> Result<()> {
    let cfg = Config { mode: 1 };
    let font = figlet_rs::FIGfont::standard().expect("there's error dude 💔🥀");

    // setup terminal
    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    // apply terminal backend
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    loop {
        terminal.draw(|f| render(f, &font, &cfg))?;
        if update()? {
            break;
        }
    }

    // ts was restoring the terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;

    Ok(())
}
