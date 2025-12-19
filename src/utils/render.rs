use crate::utils::config::*;
use anyhow::Result;
use chrono::{Local, Timelike};
use crossterm::event::{self, Event, KeyCode};
use figlet_rs::FIGfont;
use ratatui::{
    Frame,
    prelude::{Alignment, Constraint, Layout},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
};

fn get_time(rn: &chrono::DateTime<Local>, config: &Config) -> String {
    let fmt_h = match config.time_format {
        TimeFormat::Format12h => "%I",
        TimeFormat::Format24h => "%H",
    };

    let fmt_col = if rn.second() % 2 == 0 {
        format!("{}:%M", fmt_h)
    } else {
        format!("{}  %M", fmt_h)
    };

    match config.display_format {
        DisplayFormat::FormatHMS => rn.format(&format!("{}:%M:%S", fmt_h)).to_string(),
        DisplayFormat::FormatHM => rn.format(&fmt_col).to_string(),
    }
}

macro_rules! span_am_pm {
    ($am_pm:ident, $label:tt) => {
        Span::styled(
            $label,
            if $am_pm == $label {
                Style::default().add_modifier(Modifier::REVERSED)
            } else {
                Style::default()
            },
        )
    };
}

// FIXME: nunggu 1 detik dulu baru cek, jangan tiap iterasi dicek
fn handle_format12h(rn: &chrono::DateTime<Local>) -> Paragraph<'static> {
    let am_pm = rn.format("%p").to_string();

    Paragraph::new(Line::from(vec![
        Span::raw("\n"),
        span_am_pm!(am_pm, "AM"),
        Span::raw("/"),
        span_am_pm!(am_pm, "PM"),
    ]))
    .alignment(Alignment::Center)
}

pub fn render(frame: &mut Frame, font: &FIGfont, config: &Config) {
    let rn = Local::now();
    let area = frame.area();

    let chunks = Layout::vertical([
        Constraint::Min(1),
        Constraint::Length(1),
        Constraint::Length(6),
        Constraint::Length(1),
        Constraint::Min(1),
    ])
    .split(area);

    // date
    let date = rn.format("%A, %d %B %Y"); // Sunday, 12 October 2025
    let date_text = Paragraph::new(date.to_string()).alignment(Alignment::Center);

    // time
    let time = get_time(&rn, config);
    let time_figure = font.convert(&time.to_string()).unwrap();
    let time_text = Paragraph::new(time_figure.to_string()).alignment(Alignment::Center);

    // render
    frame.render_widget(date_text, chunks[1]);
    frame.render_widget(time_text, chunks[2]);

    match config.time_format {
        TimeFormat::Format12h => {
            frame.render_widget(handle_format12h(&rn), chunks[3]);
        }
        _ => {}
    }
}

pub fn update() -> Result<bool> {
    let rn = Local::now();
    let millis: u32 = rn.timestamp_subsec_millis();
    let wait: u64 = (1000 as u32 - millis) as u64;

    // if currently no event then return false
    if !event::poll(std::time::Duration::from_millis(wait))? {
        return Ok(false);
    }

    // checking key event
    if let Event::Key(key) = event::read()? {
        // if pressed key is 'q' then return true; exit the program
        if key.code == KeyCode::Char('q') {
            return Ok(true);
        }
    }

    Ok(false)
}
