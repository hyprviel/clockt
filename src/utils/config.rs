use std::{
    collections::HashMap,
    fs::File,
    io::{Read, Write},
};

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

pub fn get_settings() -> Config {
    let mut file = File::open("config.conf").unwrap_or_else(|_| {
        let mut fallback_file = File::create("config.conf").expect("unable to create file");
        let fallback_content = "timeFormat=12h\ndisplayFormat=hm\n";
        fallback_file
            .write_all(fallback_content.as_bytes())
            .expect("gabisa nulis file");

        File::open("config.conf").expect("unable to open file")
    });

    let mut content = String::with_capacity(1024);
    file.read_to_string(&mut content).expect("");

    let mut config = Config {
        time_format: TimeFormat::Format12h,
        display_format: DisplayFormat::FormatHM,
    };

    let kv_pair: HashMap<&str, &str> = content
        .lines()
        .filter_map(|line| line.split_once("="))
        .map(|(k, v)| (k.trim(), v.trim()))
        .collect();

    config.time_format = kv_pair
        .get(&"timeFormat")
        .map(|val| match *val {
            "12h" => TimeFormat::Format12h,
            "24h" => TimeFormat::Format24h,
            _ => TimeFormat::default(),
        })
        .unwrap_or_default();

    config.display_format = kv_pair
        .get(&"displayFormat")
        .map(|val| match *val {
            "12h" => DisplayFormat::FormatHM,
            "24h" => DisplayFormat::FormatHMS,
            _ => DisplayFormat::default(),
        })
        .unwrap_or_default();

    config
}
