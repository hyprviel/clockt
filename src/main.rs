mod utils;
use anyhow::{Result, anyhow};
use figlet_rs::FIGfont;
use utils::{config::*, render::*};

fn main() -> Result<()> {
    let cfg = get_settings();
    let font = FIGfont::standard().map_err(|err| anyhow!(err))?;

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
