use std::{fs::File, io::Read};

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

macro_rules! r#match {
    ($ident:ident, $ty:ty, $( $res:expr, $val:expr )*) => {
        match $ident {
            $($res => $val,)*
            _ => <$ty>::default()
        }
    };
}

pub fn get_settings() -> Config {
    let mut file = File::open("config.conf")
        .unwrap_or_else(|_| File::create("config.conf").expect("unable to create file"));

    let mut content = String::with_capacity(1024);
    file.read_to_string(&mut content)
        .expect("unable to read file");

    let mut config = Config {
        time_format: TimeFormat::Format12h,
        display_format: DisplayFormat::FormatHM,
    };

    let kv_pair: Vec<(&str, &str)> = content
        .lines()
        .filter_map(|line| line.split_once("="))
        .map(|(k, v)| (k.trim(), v.trim()))
        .collect();

    for (key, value) in kv_pair {
        match key {
            "timeFormat" => {
                config.time_format = r#match!(
                    value, TimeFormat,
                    "12h", TimeFormat::Format12h
                    "24h", TimeFormat::Format24h
                );
            }
            "displayFormat" => {
                config.display_format = r#match!(
                    value, DisplayFormat,
                    "hm", DisplayFormat::FormatHM
                    "hms", DisplayFormat::FormatHMS
                )
            }
            _ => {}
        }
    }

    config
}
