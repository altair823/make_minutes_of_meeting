use chrono::{DateTime, Local};
use log::{error, info, warn, LevelFilter};
use serde_json::json;
use std::env;
use std::fs;
use std::io::{Error, Write};
use std::path::Path;
use whoami;

#[derive(Default)]
struct Config {
    filename: String,
    datetime: DateTime<Local>,
    author: String,
    header: String,
    footer: String,
    extension: String,
}

impl Config {
    fn new() -> Config {
        Config::default()
    }

    fn set_filename(&mut self, filename: String) {
        self.filename = filename;
    }

    fn set_datetime(&mut self, datetime: DateTime<Local>) {
        self.datetime = datetime;
    }

    fn set_author(&mut self, author: String) {
        self.author = author;
    }

    fn set_header(&mut self, header: String) {
        self.header = header;
    }

    fn set_footer(&mut self, footer: String) {
        self.footer = footer;
    }

    fn set_extension(&mut self, extension: String) {
        self.extension = extension;
    }
}

fn main() -> Result<(), Error> {
    let current_exe_dir = env::current_exe().expect("Error getting current executable directory").parent().expect("Error getting parent directory").to_path_buf();
    simple_logging::log_to_file(format!("{}/log.txt", current_exe_dir.display()).as_str(), LevelFilter::Info).expect("Error setting up logging");
    let datetime = Local::now();
    let mut config = Config::new();

    let args: Vec<String> = env::args().collect();
    let filename;
    if args.len() < 2 {
        info!("No filename provided. Using default filename");
        filename = "default";
    } else {
        info!("Filename provided: {}", &args[1]);
        filename = &args[1];
    }
    info!("Filename: {}", filename);
    config.set_filename(filename.to_string());
    config.set_datetime(datetime);
    info!(
        "DateTime: {}",
        config.datetime.format("%Y-%m-%d %H:%M:%S").to_string()
    );

    let config_file_path = current_exe_dir.join("config.json");
    if Path::is_file(&config_file_path) {
        info!("Config file exists")
    } else {
        println!("Config file does not exist. Creating default config file...");
        warn!("Config file does not exist. Creating default config file...");
        let default_username = whoami::username();
        info!("Default username: {}", default_username);
        let default_config = json!({});
        let default_config = serde_json::to_string_pretty(&default_config).unwrap_or_else(|_| {
            error!("Error serializing default config");
            panic!("Error serializing default config");
        });
        fs::write(&config_file_path, default_config).unwrap_or_else(|_| {
            error!("Error writing default config file");
            panic!("Error writing default config file");
        });
    }
    let config_string = fs::read_to_string(&config_file_path).expect("Error reading config file");
    let config_json: serde_json::Value =
        serde_json::from_str(&config_string).expect("Error parsing config file");
    config.set_author(
        config_json["author"]
            .as_str()
            .unwrap_or(whoami::username().as_str())
            .to_string(),
    );
    config.set_header(
        config_json["header"]
            .as_str()
            .unwrap_or_default()
            .to_string(),
    );
    config.set_footer(
        config_json["footer"]
            .as_str()
            .unwrap_or_default()
            .to_string(),
    );
    config.set_extension(
        config_json["extension"]
            .as_str()
            .unwrap_or("txt")
            .to_string(),
    );

    let mut new_file = fs::File::create(format!("{}.{}", &config.filename, &config.extension))?;
    let datetime_string = config.datetime.format("%Y-%m-%d %H:%M:%S").to_string();
    new_file.write(
        format!(
            "{}",
            match config.extension.as_str() {
                "md" => "# ",
                _ => "",
            }
        )
        .as_bytes(),
    )?;
    new_file.write(format!("{}\n\n", &config.filename).as_bytes())?;
    new_file.write(
        format!(
            "DateTime: {}\n\
    author: {}\n\
    ",
            datetime_string, &config.author
        )
        .as_bytes(),
    )?;

    new_file.write(format!("{}\n", &config.header).as_bytes())?;

    new_file.write(format!("{}", &config.footer).as_bytes())?;

    new_file.flush()?;
    Ok(())
}
