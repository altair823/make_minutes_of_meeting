use std::path::Path;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, author, about, long_about)]
pub struct Cli {
    pub filename: String,
    #[arg(short, long, default_value_t = false)]
    pub overwrite: bool,
    #[arg(short, long, default_value_t = false)]
    pub verbose: bool,
    #[arg(short, long)]
    pub author: Option<String>,
}

impl Cli {
    pub fn get_extension_if_exists(&self) -> Option<String> {
        let filename = Path::new(&self.filename);
        match filename.extension() {
            Some(ext) => Some(ext.to_str().unwrap().to_string()),
            None => None,
        }
    }
}