
mod cli;
pub mod config;
mod log_initializer;
pub mod metadata;
mod mom_opener;


use chrono::Local;
use clap::Parser;
use log::{error, info, warn};
use std::{env, io};
use std::fs;
use std::io::Error;

use cli::Cli;
use config::Config;
use crate::metadata::Metadata;
use crate::mom_opener::Moms;


fn make_metadata(cli: &Cli) -> Metadata {
    let current_exe_dir = env::current_exe()
        .expect("Error getting current executable directory")
        .parent()
        .expect("Error getting parent directory")
        .to_path_buf();
    let config_file_path = &current_exe_dir.join("config.json");
    let config = Config::from_file(&config_file_path).unwrap_or_else(|_| {
        warn!("Error loading config file");
        Config::create_blank_config_file(&config_file_path).unwrap();
        Config::from_file(&config_file_path).unwrap()
    });
    info!("Config loaded successfully");
    let metadata = Metadata::from(&cli, &config);
    metadata
}

fn main() -> Result<(), Error> {
    let cli = Cli::parse();

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

    let metadata = make_metadata(&cli);

    let new_file_path = env::current_dir()?.join(match &metadata.extension {
        Some(extension) => format!("{}.{}", &metadata.filestem, extension),
        None => metadata.filestem.clone(),
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
    let mut new_file = match new_file_options.open(&new_file_path) {
        Ok(file) => file,
        Err(e) => match e.kind() {
            io::ErrorKind::AlreadyExists => {
                error!("File already exists. Use -o to overwrite");
                panic!("File already exists. Use -o to overwrite");
            },
            e => {
                error!("Error opening file: {}", e);
                panic!("Error opening file: {}", e);
            },
        }
    };

    metadata.to_file(&mut new_file).unwrap_or_else(|_| {
        error!("Error writing metadata to file");
        panic!("Error writing metadata to file");
    });

    info!("Trying to open file with default program");
    let moms = Moms { moms: vec![new_file_path.to_str().unwrap().to_string()] };
    moms.open_all();


    info!(
        "--------End logging at {}--------",
        Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
    );
    Ok(())
}
