use std::path::Path;
use clap::Parser;

#[derive(Parser, Debug, Default)]
#[command(version, about, long_about)]
pub struct Cli {
    /// The filename of the new document
    pub filename: String,
    /// Overwrite the file if it already exists
    #[arg(short, long, default_value_t = false)]
    pub overwrite: bool,
    #[arg(short, long, default_value_t = false)]
    pub verbose: bool,
    /// The author of the document
    #[arg(short, long)]
    pub author: Option<String>,
    /// Open all files after creating them
    #[arg(short = 'p', long)]
    pub open: bool,
    /// Add additional metadata to the document
    #[arg(short, long, default_value_t = false)]
    pub enrich: bool,
    /// Create a default configuration file
    #[arg(long)]
    pub create_config: bool,
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