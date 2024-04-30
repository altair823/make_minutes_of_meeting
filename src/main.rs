mod cli;
mod config;
mod log_initializer;

use chrono::Local;
use clap::Parser;
use log::{error, info, warn};
use std::env;
use std::fs;
use std::io::{Error, Write};
use std::path::PathBuf;
use whoami;

use cli::Cli;
use config::Config;

fn determine_filestem(cli: &Cli) -> String {
    let filename = &cli.filename;
    let filestem = PathBuf::from(&filename)
        .file_stem()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();
    filestem
}

fn determine_extension(config: &Config, cli: &Cli) -> Option<String> {
    match PathBuf::from(&cli.filename).extension() {
        Some(extension) => {
            info!("Extension provided: {}", extension.to_str().unwrap());
            Some(extension.to_str().unwrap().to_string())
        }
        None => {
            info!("Extension not provided. Using extension from config file");
            match &config.extension {
                Some(extension) => Some(extension.clone()),
                None => {
                    warn!("Extension not found in config file. Not Using extension.");
                    None
                }
            }
        }
    }
}

fn determine_author(config: &Config, cli: &Cli) -> String {
    match &cli.author {
        Some(author) => {
            info!("Author provided: {}", author);
            author.clone()
        }
        None => {
            info!("Author not provided. Using author from config file");
            match &config.author {
                Some(author) => author.clone(),
                None => {
                    warn!("Author not found in config file. Using current user to author");
                    whoami::username()
                }
            }
        }
    }
}

fn make_config(cli: &Cli) -> Config {
    let current_exe_dir = env::current_exe()
        .expect("Error getting current executable directory")
        .parent()
        .expect("Error getting parent directory")
        .to_path_buf();

    let mut config = Config::from_file(&current_exe_dir.join("config.json")).unwrap_or_else(|_| {
        error!("Error loading config file");
        panic!("Error loading config file");
    });
    info!("Config loaded successfully");

    let filestem = determine_filestem(&cli);
    info!("Filestem: {}", &filestem);
    config.set_filestem(&filestem);

    let extension = determine_extension(&config, &cli);
    info!(
        "Extension: {}",
        match &extension {
            Some(extension) => extension,
            None => "None",
        }
    );
    config.set_extension(extension);

    let author = determine_author(&config, &cli);
    info!("Author: {}", &author);
    config.set_author(author);

    config
}

fn write_metadata(
    new_file: &mut fs::File,
    config: &Config,
    datetime_string: String,
) -> Result<(), Error> {
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
    new_file.write(format!("{}\n\n", &config.filestem.clone().unwrap()).as_bytes())?;
    new_file.write(
        format!(
            "created: {}\n\
    author: {}\n\
    ",
            datetime_string,
            &config.author.clone().unwrap()
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

    log_initializer::init(&current_exe_dir, &cli);

    info!(
        "--------Start logging at {}--------",
        Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
    );

    let config = make_config(&cli);
    let datetime_string = datetime.format("%Y-%m-%d %H:%M:%S").to_string();

    let new_file_path = env::current_dir()?.join(match &config.extension {
        Some(extension) => format!("{}.{}", &config.filestem.clone().unwrap(), extension),
        None => config.filestem.clone().unwrap(),
    });
    let mut new_file_options = fs::OpenOptions::new();
    match &cli.overwrite {
        true => {
            info!("Overwriting flag set true. Overwriting file if exists");
            new_file_options.write(true).create(true).truncate(true);
        }
        false => {
            info!("Not overwriting file");
            new_file_options.write(true).create_new(true);
        }
    }
    let mut new_file = new_file_options.open(&new_file_path).unwrap_or_else(|_| {
        error!("Error opening new file");
        panic!("Error opening new file");
    });

    write_metadata(&mut new_file, &config, datetime_string).unwrap_or_else(|_| {
        error!("Error writing metadata to new file");
        panic!("Error writing metadata to new file");
    });
    info!(
        "--------End logging at {}--------",
        Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
    );
    Ok(())
}
