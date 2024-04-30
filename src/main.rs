mod config;
mod cli;
mod log_initializer;

use chrono::Local;
use log::{error, info, warn};
use serde_json::json;
use std::env;
use std::fs;
use std::io::{Error, Write};
use std::path::{Path, PathBuf};
use clap::Parser;
use whoami;

use config::Config;
use cli::Cli;

fn load_config_from_file<P: AsRef<Path>>(current_exe_dir: P, filename: &str) -> Config {
    let config_file_path = current_exe_dir.as_ref().join("config.json");
    if Path::is_file(&config_file_path) {
        info!("Config file exists. Reading config file...")
    } else {
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
    let config_string = fs::read_to_string(&config_file_path).unwrap_or_else(|_| {
        error!("Error reading config file");
        panic!("Error reading config file");
    });
    let config_json: serde_json::Value =
        serde_json::from_str(&config_string).unwrap_or_else(|_| {
            error!("Error parsing config file");
            panic!("Error parsing config file");
        });

    let mut config = Config::new();

    info!("Filename provided: {}", filename);
    match Path::new(&filename).extension() {
        Some(extension) => {
            info!("Extension: {}", extension.to_str().unwrap());
            config.set_extension(Some(extension.to_str().unwrap().to_string()));
        }
        None => {
            info!("Extension not found in filename. Try Using preconfigured extension");
            match config_json["extension"].as_str() {
                Some(extension) => config.set_extension(Some(extension.to_string())),
                None => {
                    warn!("Extension not found in config file. Not Using extension.");
                    config.set_extension(None);
                }
            };
        }
    };
    let filestem = PathBuf::from(&filename).file_stem().unwrap().to_str().unwrap().to_string();
    config.set_filestem(&filestem);
    info!("Filestem: {}", &filestem);
    info!("Extension: {}", match config.extension {
        Some(ref extension) => extension,
        None => "None",

    });


    config.set_author(match config_json["author"].as_str() {
        Some(author) => author.to_string(),
        None => {
            warn!("Author not found in config file. Using current user to author");
            whoami::username()
        }
    });
    config.set_header(match config_json["header"].as_str() {
        Some(header) => Some(header.to_string()),
        None => {
            warn!("Header not found in config file.");
            None
        }
    });
    config.set_footer(match config_json["footer"].as_str() {
        Some(footer) => Some(footer.to_string()),
        None => {
            warn!("Footer not found in config file.");
            None
        }
    });

    config
}

fn write_metadata(new_file: &mut fs::File, config: &Config, datetime_string: String) -> Result<(), Error> {
    new_file.write(
        format!(
            "{}",
            match config.extension {
                Some(ref extension) => match extension.as_str() {
                    "md" => "# ".to_string(),
                    _ => "".to_string(),
                },
                None => "".to_string(),
            }
        )
            .as_bytes(),
    )?;
    new_file.write(format!("{}\n\n", &config.filestem).as_bytes())?;
    new_file.write(
        format!(
            "created: {}\n\
    author: {}\n\
    ",
            datetime_string, &config.author
        )
            .as_bytes(),
    )?;

    new_file.write(
        format!(
            "{}\n",
            match &config.header {
                Some(header) => header,
                None => "",
            }
        )
            .as_bytes(),
    )?;

    new_file.write("\n".as_bytes())?;

    new_file.write(
        format!(
            "{}",
            match &config.footer {
                Some(footer) => footer,
                None => "",
            }
        )
            .as_bytes(),
    )?;

    new_file.flush()?;
    Ok(())
}

fn main() -> Result<(), Error> {
    let cli = Cli::parse();

    let datetime = Local::now();
    info!(
        "DateTime: {}",
        datetime.format("%Y-%m-%d %H:%M:%S").to_string()
    );

    let current_exe_dir = env::current_exe()
        .expect("Error getting current executable directory")
        .parent()
        .expect("Error getting parent directory")
        .to_path_buf();

    log_initializer::init(&current_exe_dir);

    info!(
        "--------Start logging at {}--------",
        Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
    );

    let mut config = load_config_from_file(&current_exe_dir, &cli.filename);
    match cli.author {
        Some(author) => {
            info!("Author provided: {}", author);
            config.set_author(author);
        }
        None => {
            info!("Author not provided. Using author from config file");
        }
    }

    let mut new_file;
    let full_filename = match config.extension {
        Some(ref extension) => format!("{}.{}", &config.filestem, extension),
        None => format!("{}", &config.filestem),
    };
    match fs::File::create(&full_filename) {
        Ok(file) => {
            info!(
                "File created successfully in \"{}\"",
                env::current_dir()
                    .unwrap_or(PathBuf::from("unknown"))
                    .join(&full_filename)
                    .to_str()
                    .unwrap_or("unknown")
            );
            new_file = file;
        }
        Err(e) => {
            error!("Error creating file: {}", e);
            panic!("Error creating file: {}", e);
        }
    };
    let datetime_string = datetime.format("%Y-%m-%d %H:%M:%S").to_string();

    write_metadata(&mut new_file, &config, datetime_string)?;

    info!(
        "--------End logging at {}--------",
        Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
    );
    Ok(())
}
