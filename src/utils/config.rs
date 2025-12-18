use std::{fs::File, io::Read};

pub enum DisplayFormat {
    FormatHM,
    FormatHMS,
}

pub enum TimeFormat {
    Format12h,
    Format24h,
}

pub struct Config {
    pub time_format: TimeFormat,
    pub display_format: DisplayFormat,
}

// TODO: ew, gross! refactor this thing soon!
pub fn get_settings() -> Config {
    let mut file = File::open("config.conf").expect("error opening file");
    let mut content = String::with_capacity(1024);
    let _ = file
        .read_to_string(&mut content)
        .map_err(|err| panic!("{}", err));

    let mut display_format = DisplayFormat::FormatHM; // default
    let mut time_format = TimeFormat::Format12h; // default

    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue; // skip comment atau baris kosong
        }
        if let Some((key, value)) = line.split_once('=') {
            match key.trim() {
                "timeFormat" => match value.trim() {
                    "12h" => time_format = TimeFormat::Format12h,
                    "24h" => time_format = TimeFormat::Format24h,
                    _ => (),
                },
                "displayFormat" => match value.trim().to_lowercase().as_str() {
                    "hms" => display_format = DisplayFormat::FormatHMS,
                    "hm" => display_format = DisplayFormat::FormatHM,
                    _ => (),
                },
                _ => (),
            }
        }
    }

    Config {
        time_format: time_format,
        display_format: display_format,
    }
}
