mod utils;
use crate::utils::{Config, render, update};
use anyhow::Result;

fn main() -> Result<()> {
    let cfg = Config { mode: 1 };
    let font = figlet_rs::FIGfont::standard().expect("there's error dude 💔🥀");

    // setup terminal
    let mut terminal = ratatui::init();
    loop {
        terminal.draw(|f| render(f, &font, &cfg))?;
        if update()? {
            break;
        }
    }
    ratatui::restore();

    Ok(())
}
