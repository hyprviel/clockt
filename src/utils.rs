use anyhow::Result;
use chrono::{Local, Timelike};
use crossterm::event::{self, Event, KeyCode};
use figlet_rs::FIGfont;
use ratatui::{
    Frame,
    prelude::{Alignment, Constraint, Layout},
    widgets::Paragraph,
};
use std::time::Duration;

pub struct Config {
    pub mode: u8,
}

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
    let second = rn.second();
    let time = match config.mode {
        1 => rn.format("%H:%M:%S").to_string(), // 12:46:40
        2 => rn
            .format(if second % 2 == 0 { "%H:%M" } else { "%H  %M" })
            .to_string(),
        _ => String::new(),
    };

    let time_figure = font.convert(&time.to_string()).unwrap();
    let time_text = Paragraph::new(time_figure.to_string()).alignment(Alignment::Center);

    frame.render_widget(date_text, chunks[1]);
    frame.render_widget(time_text, chunks[2]);
}

pub fn update() -> Result<bool> {
    let rn = Local::now();
    let millis: u32 = rn.timestamp_subsec_millis();
    let wait: u64 = (1000 as u32 - millis) as u64;

    if !event::poll(Duration::from_millis(wait))? {
        return Ok(false);
    }

    if let Event::Key(key) = event::read()? {
        if key.code == KeyCode::Char('q') {
            return Ok(true);
        }
    }

    Ok(false)
}
