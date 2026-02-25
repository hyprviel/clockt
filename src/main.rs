mod utils;
use anyhow::{Result, anyhow};
use figlet_rs::FIGfont;
use utils::{
    config::Config,
    flags::get_flags,
    render::{render, update},
};

fn main() -> Result<()> {
    let font = FIGfont::standard().map_err(|err| anyhow!(err))?;
    let cfg = get_flags().unwrap_or(Config::default());

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
