pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub mod graphics {
    use super::Result;
    use figlet_rs::FIGfont;
    use std::time::Duration;
    use chrono::{Local, Timelike};
    use crossterm::event::{self, Event, KeyCode};
    use ratatui::{
        Frame,
        prelude::{Alignment, Constraint, Layout},
        widgets::Paragraph,
    };

    pub struct Config { pub mode: u8 }

    pub fn render(frame: &mut Frame, font: &FIGfont, config: &Config) {
        let area = frame.area();
        let rn = Local::now();

        let chunks = Layout::vertical([
            Constraint::Min(1),
            Constraint::Length(1),
            Constraint::Length(5),
            Constraint::Min(1),
        ])
        .split(area);

        // date
        let date = rn.format("%A, %d %B %Y"); // Sunday, 12 October 2025
        let date_text = Paragraph::new(date.to_string()).alignment(Alignment::Center);

        // time
        let mut time = String::new();
        if config.mode == 1 {
            time = rn.format("%H:%M:%S").to_string(); // 12:46:40
        } else if config.mode == 2 {
            let second = rn.second();
            time = if second % 2 == 0 {
                rn.format("%H:%M").to_string()
            } else {
                rn.format("%H  %M").to_string()
            };
        }

        let time_figure = font.convert(&time.to_string()).unwrap();
        let time_text = Paragraph::new(time_figure.to_string()).alignment(Alignment::Center);

        frame.render_widget(date_text, chunks[1]);
        frame.render_widget(time_text, chunks[2]);
    }

    pub fn update() -> Result<bool> {
        let rn = Local::now();
        let millis: u32 = rn.timestamp_subsec_millis();
        let wait: u64 = (1000 as u32 - millis) as u64;

        if event::poll(Duration::from_millis(wait))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    return Ok(true);
                }
            }
        }

        Ok(false)
    }
}
