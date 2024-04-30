use log::warn;
use serde::Serialize;
use serde_derive::Deserialize;
use std::error::Error;
use std::fs;
use std::path::Path;

#[derive(Default, Serialize, Deserialize, PartialOrd, PartialEq, Debug)]
pub struct Config {
    pub filestem: Option<String>,
    pub author: Option<String>,
    pub header: Option<String>,
    pub footer: Option<String>,
    pub extension: Option<String>,
}

impl Config {
    pub fn new() -> Self {
        Config::default()
    }

    pub fn set_filestem(&mut self, filename: &str) {
        self.filestem = Some(filename.to_string());
    }

    pub fn set_author(&mut self, author: String) {
        self.author = Some(author);
    }

    pub fn set_header(&mut self, header: Option<String>) {
        self.header = header;
    }

    pub fn set_footer(&mut self, footer: Option<String>) {
        self.footer = Option::from(footer);
    }

    pub fn set_extension(&mut self, extension: Option<String>) {
        self.extension = extension;
    }

    fn create_blank_config_file<P: AsRef<Path>>(config_file: P) -> Result<(), Box<dyn Error>> {
        let config_json = String::from("{}");
        fs::write(&config_file, config_json)?;
        Ok(())
    }

    fn create_config_file<P: AsRef<Path>>(&self, config_file: P) -> Result<(), Box<dyn Error>> {
        let config_json = serde_json::to_string_pretty(self)?;
        fs::write(&config_file, config_json)?;
        Ok(())
    }

    pub fn from_file<P: AsRef<Path>>(config_file: P) -> Result<Self, Box<dyn Error>> {
        if !config_file.as_ref().exists() {
            warn!("Config file not found. Create a new one.");
            Config::create_blank_config_file(&config_file)?;
        }
        let config_json = fs::read_to_string(&config_file)?;
        let config: Config = serde_json::from_str(&config_json)?;
        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Read;
    use tempfile::tempdir;

    #[test]
    fn test_set_filestem() {
        let mut config = Config::new();
        config.set_filestem("test");
        assert_eq!(config.filestem.unwrap(), "test");
    }

    #[test]
    fn test_set_author() {
        let mut config = Config::new();
        config.set_author("test".to_string());
        assert_eq!(config.author.unwrap(), "test");
    }

    #[test]
    fn test_set_header() {
        let mut config = Config::new();
        config.set_header(Some("test".to_string()));
        assert_eq!(config.header.unwrap(), "test".to_string());
    }

    #[test]
    fn test_set_footer() {
        let mut config = Config::new();
        config.set_footer(Some("test".to_string()));
        assert_eq!(config.footer.unwrap(), "test".to_string());
    }

    #[test]
    fn test_set_extension() {
        let mut config = Config::new();
        config.set_extension(Some("test".to_string()));
        assert_eq!(config.extension.unwrap(), "test".to_string());
    }

    #[test]
    fn test_create_blank_config_file() {
        let dir = tempdir().unwrap();
        let config_file = dir.path().join("config.json");
        Config::create_blank_config_file(&config_file).unwrap();
        let mut file = File::open(&config_file).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        assert_eq!(contents, "{}");
    }

    #[test]
    fn test_create_config_file() {
        let dir = tempdir().unwrap();
        let config_file = dir.path().join("config.json");
        let mut config = Config::new();
        config.set_filestem("test_filestem");
        config.set_author("test_author".to_string());
        config.set_header(Some("test_header".to_string()));
        config.set_footer(Some("test_footer".to_string()));
        config.set_extension(Some("test_extension".to_string()));
        config.create_config_file(&config_file).unwrap();
        let mut file = File::open(&config_file).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        assert_eq!(contents, serde_json::to_string_pretty(&config).unwrap());
    }

    #[test]
    fn test_from_file() {
        let dir = tempdir().unwrap();
        let config_file = dir.path().join("config.json");
        let mut config = Config::new();
        config.set_filestem("test_filestem");
        config.set_author("test_author".to_string());
        config.set_header(Some("test_header".to_string()));
        config.set_footer(Some("test_footer".to_string()));
        config.set_extension(Some("test_extension".to_string()));
        config.create_config_file(&config_file).unwrap();
        let config_from_file = Config::from_file(&config_file).unwrap();
        assert_eq!(config, config_from_file);
    }

    #[test]
    fn test_from_file_with_empty_key() {
        let dir = tempdir().unwrap();
        let config_file = dir.path().join("config.json");
        Config::create_blank_config_file(&config_file).unwrap();
        let config = Config::new();
        let config_from_file = Config::from_file(&config_file).unwrap();
        assert_eq!(config, config_from_file);
    }

    #[test]
    fn test_from_file_with_missing_file() {
        let dir = tempdir().unwrap();
        let config_file = dir.path().join("config.json");
        let config = Config::new();
        let config_from_file = Config::from_file(&config_file).unwrap();
        assert_eq!(config, config_from_file);
    }
}
