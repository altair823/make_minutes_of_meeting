use crate::cli::Cli;
use log::LevelFilter;
use simplelog::CombinedLogger;
use std::fs;
use std::path::Path;

pub fn init<P: AsRef<Path>>(current_exe_dir: P, cli: &Cli) {
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

    let term_logger = match &cli.verbose {
        true => simplelog::TermLogger::new(
            LevelFilter::Info,
            log_config.clone(),
            simplelog::TerminalMode::Mixed,
            simplelog::ColorChoice::Auto,
        ),
        false => simplelog::TermLogger::new(
            LevelFilter::Warn,
            log_config.clone(),
            simplelog::TerminalMode::Mixed,
            simplelog::ColorChoice::Never,
        ),
    };
    let _ = CombinedLogger::init(vec![
        term_logger,
        simplelog::WriteLogger::new(LevelFilter::Info, log_config.clone(), log_file),
    ]);
}
