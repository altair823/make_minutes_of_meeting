use chrono::{DateTime, Local};

#[derive(Default)]
pub struct Config {
    pub filename: String,
    pub datetime: DateTime<Local>,
    pub author: String,
    pub header: Option<String>,
    pub footer: Option<String>,
    pub extension: Option<String>,
}

impl Config {
    pub fn new() -> Config {
        Config::default()
    }

    pub fn set_filename(&mut self, filename: String) {
        self.filename = filename;
    }

    pub fn set_datetime(&mut self, datetime: DateTime<Local>) {
        self.datetime = datetime;
    }

    pub fn set_author(&mut self, author: String) {
        self.author = author;
    }

    pub fn set_header(&mut self, header: Option<String>) {
        self.header = header;
    }

    pub fn set_footer(&mut self, footer: Option<String>) {
        self.footer = Option::from(footer);
    }

    pub fn set_extension(&mut self, extension: String) {
        self.extension = Option::from(extension);
    }
}