use super::config::*;
use std::env;

pub fn get_flags() -> Option<super::config::Config> {
    let mut res = Config::default();

    env::args().for_each(|i| match i.as_str() {
        "-t12" => res.time_format = TimeFormat::Format12h,
        "-t24" => res.time_format = TimeFormat::Format24h,
        "-dHM" => res.display_format = DisplayFormat::FormatHM,
        "-dHMS" => res.display_format = DisplayFormat::FormatHMS,
        _ => {}
    });

    Some(res)
}
