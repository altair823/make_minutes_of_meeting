mod config;

use chrono::Local;
use log::{error, info, warn, LevelFilter};
use serde_json::json;
use simplelog::CombinedLogger;
use std::env;
use std::fs;
use std::io::{Error, Write};
use std::path::{Path, PathBuf};
use whoami;

use config::Config;

fn setup_logger<P: AsRef<Path>>(current_exe_dir: P) {
    let log_file_path = current_exe_dir.as_ref().join("mmomlog.log");
    let log_file = match Path::is_file(&log_file_path) {
        true => fs::OpenOptions::new()
            .read(true)
            .append(true)
            .open(&log_file_path)
            .unwrap(),
        false => fs::File::create(&log_file_path).unwrap(),
    };

    let log_config = simplelog::ConfigBuilder::new()
        .set_time_offset_to_local()
        .unwrap()
        .set_level_color(simplelog::Level::Error, Some(simplelog::Color::Red))
        .set_level_color(simplelog::Level::Warn, Some(simplelog::Color::Yellow))
        .set_level_color(simplelog::Level::Info, Some(simplelog::Color::Green))
        .set_level_color(simplelog::Level::Debug, Some(simplelog::Color::Blue))
        .set_level_color(simplelog::Level::Trace, Some(simplelog::Color::Magenta))
        .build();

    let _ = CombinedLogger::init(vec![
        simplelog::TermLogger::new(
            LevelFilter::Info,
            log_config.clone(),
            simplelog::TerminalMode::Mixed,
            simplelog::ColorChoice::Auto,
        ),
        simplelog::WriteLogger::new(LevelFilter::Info, log_config.clone(), log_file),
    ]);
}

fn load_config_from_file<P: AsRef<Path>>(current_exe_dir: P, config_filename: Option<String>) -> Config {
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

    let datetime = Local::now();
    let mut config = Config::new();

    let args: Vec<String> = env::args().collect();
    let mut filename;
    match config_filename {
        Some(config_filename) => {
            info!("Filename provided: {}", &args[1]);
            filename = config_filename;
            match Path::new(&filename).extension() {
                Some(extension) => {
                    info!("Extension: {}", extension.to_str().unwrap());
                    config.set_extension(extension.to_str().unwrap().to_string());
                    filename = PathBuf::from(filename).file_stem().unwrap().to_str().unwrap().to_string();
                }
                None => {
                    info!("Extension not found in filename. Try Using preconfigured extension");
                    match config_json["extension"].as_str() {
                        Some(extension) => config.set_extension(extension.to_string()),
                        None => {
                            warn!("Extension not found in config file. Not Using extension!");
                        }
                    };
                }
            };
        }
        None => {
            info!("No filename provided. Using filename \"default\"");
            filename = "default".to_string();
        }
    }
    info!("Filename: {}", filename);
    config.set_filename(filename.to_string());
    config.set_datetime(datetime);
    info!(
        "DateTime: {}",
        config.datetime.format("%Y-%m-%d %H:%M:%S").to_string()
    );

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
    new_file.write(format!("{}\n\n", &config.filename).as_bytes())?;
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
    let current_exe_dir = env::current_exe()
        .expect("Error getting current executable directory")
        .parent()
        .expect("Error getting parent directory")
        .to_path_buf();

    setup_logger(&current_exe_dir);

    info!(
        "--------Start logging at {}--------",
        Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
    );

    let args: Vec<String> = env::args().collect();
    let config = load_config_from_file(&current_exe_dir, match args.len() > 1 {
        true => Some(args[1].clone()),
        false => None,
    });

    let mut new_file;
    let full_filename = match config.extension {
        Some(ref extension) => format!("{}.{}", &config.filename, extension),
        None => format!("{}", &config.filename),
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
    let datetime_string = config.datetime.format("%Y-%m-%d %H:%M:%S").to_string();

    write_metadata(&mut new_file, &config, datetime_string)?;

    info!(
        "--------End logging at {}--------",
        Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
    );
    Ok(())
}
