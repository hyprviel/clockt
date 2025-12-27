#![allow(unused)]
use std::{collections::HashMap, fs::File, io::Read};

#[derive(Default)]
pub enum DisplayFormat {
    #[default]
    FormatHM,
    FormatHMS,
}

#[derive(Default)]
pub enum TimeFormat {
    #[default]
    Format12h,
    Format24h,
}

pub struct Config {
    pub time_format: TimeFormat,
    pub display_format: DisplayFormat,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            time_format: TimeFormat::default(),
            display_format: DisplayFormat::default(),
        }
    }
}
