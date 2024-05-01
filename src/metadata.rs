//! The `metadata` module is responsible for creating and managing the metadata of the file.
//!
//!

use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use chrono::{DateTime, Local};
use log::{info, warn};
use crate::cli::Cli;
use crate::config::Config;

/// Metadata struct to hold the metadata of the file.
#[derive(Default, Debug)]
pub struct Metadata {
    /// The filestem of the file without the extension
    pub filestem: String,
    /// The author of the file
    pub author: String,
    /// The date and time the file was created
    pub datetime: DateTime<Local>,
    /// The extension of the file
    pub extension: Option<String>,
    /// The header of the file
    pub header: Option<String>,
    /// The footer of the file
    pub footer: Option<String>,
}

impl Metadata {

    /// Determine the filestem from the given filename
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

    /// Determine the extension from the given filename or the config file.
    /// - If the extension is not provided in the filename, use the extension from the config file.
    /// - If the extension is not provided in the config file, do not use the extension.
    /// - If the extension is provided in the filename, use the extension from the filename.
    /// - If the extension is provided in the filename and the config file, use the extension from the filename.
    fn determine_extension(cli: &Cli, config: &Config) -> Option<String> {
        match cli.get_extension_if_exists() {
            Some(extension) => {
                info!("Extension provided: {}", extension);
                Some(extension)
            }
            None => {
                info!("Extension not provided. Using extension from config file");
                match &config.extension {
                    Some(extension) => Some(extension.clone()),
                    None => {
                        warn!("Extension not found in config file. Not using extension");
                        None
                    }
                }
            }
        }
    }

    /// Determine the author from the given author option or the config file.
    /// - If the author is provided in the author option, use the author from the author option.
    /// - If the author is not provided in the author option, use the author from the config file.
    /// - If the author is not provided in the author option and the config file, use the current user's name.
    fn determine_author(cli: &Cli, config: &Config) -> String {
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

    /// Create a new Metadata struct from the given Cli and Config.
    pub fn from(cli: &Cli, config: &Config) -> Self {
        let filestem = Metadata::determine_filestem(cli);
        let author = Metadata::determine_author(cli, config);
        let datetime = Local::now();
        let extension = Metadata::determine_extension(cli, config);
        Metadata {
            filestem,
            author,
            datetime,
            extension,
            header: config.header.clone(),
            footer: config.footer.clone(),
        }
    }

    /// Write the metadata to the given file.
    pub fn to_file(&self, new_file: &mut File) -> Result<(), std::io::Error>{
        new_file.write(
            format!(
                "{}",
                match self.extension {
                    Some(ref extension) => match extension.as_str() {
                        "md" => "# ".to_string(),
                        _ => "".to_string(),
                    },
                    None => "".to_string(),
                }
            )
                .as_bytes(),
        )?;
        new_file.write(format!("{}\n\n", &self.filestem).as_bytes())?;
        new_file.write(
            format!(
                "created: {}\n\
    author: {}\n\n",
                self.datetime.format("%Y-%m-%d %H:%M:%S"),
                &self.author
            )
                .as_bytes(),
        )?;

        new_file.write(
            format!(
                "{}\n",
                match &self.header {
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
                match &self.footer {
                    Some(footer) => footer,
                    None => "",
                }
            )
                .as_bytes(),
        )?;

        new_file.flush()?;
        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_determine_filestem() {
        let cli = Cli {
            filename: "test.txt".to_string(),
            overwrite: false,
            verbose: false,
            author: Option::from("testuser".to_string()),
        };
        let filestem = Metadata::determine_filestem(&cli);
        assert_eq!(filestem, "test");

        let cli = Cli {
            filename: "test".to_string(),
            overwrite: false,
            verbose: false,
            author: Option::from("testuser".to_string()),
        };
        let filestem = Metadata::determine_filestem(&cli);
        assert_eq!(filestem, "test");
    }

    #[test]
    fn test_determine_extension_in_filename() {
        let cli = Cli {
            filename: "test.txt".to_string(),
            overwrite: false,
            verbose: false,
            author: Option::from("testuser".to_string()),
        };
        let config = Config::new();
        let extension = Metadata::determine_extension(&cli, &config);
        assert!(extension.is_some_and(|ext| ext == "txt"));
    }

    #[test]
    fn test_determine_extension_is_none() {
        let cli = Cli {
            filename: "test".to_string(),
            overwrite: false,
            verbose: false,
            author: Option::from("testuser".to_string()),
        };
        let config = Config::new();
        let extension = Metadata::determine_extension(&cli, &config);
        assert!(extension.is_none());
    }

    #[test]
    fn test_determine_extension_in_config() {
        let cli = Cli {
            filename: "test".to_string(),
            overwrite: false,
            verbose: false,
            author: Option::from("testuser".to_string()),
        };
        let mut config = Config::new();
        config.extension = Some("md".to_string());
        let extension = Metadata::determine_extension(&cli, &config);
        assert!(extension.is_some_and(|ext| ext == "md"));
    }

    #[test]
    fn test_determine_extension_in_filename_and_config() {
        let cli = Cli {
            filename: "test.md".to_string(),
            overwrite: false,
            verbose: false,
            author: Option::from("testuser".to_string()),
        };
        let mut config = Config::new();
        config.extension = Some("txt".to_string());
        let extension = Metadata::determine_extension(&cli, &config);
        assert!(extension.is_some_and(|ext| ext == "md"));
    }

    #[test]
    fn test_determine_author_in_cli() {
        let cli = Cli {
            filename: "test".to_string(),
            overwrite: false,
            verbose: false,
            author: Option::from("testuser".to_string()),
        };
        let config = Config::new();
        let author = Metadata::determine_author(&cli, &config);
        assert_eq!(author, "testuser");
    }

    #[test]
    fn test_determine_author_in_config() {
        let cli = Cli {
            filename: "test".to_string(),
            overwrite: false,
            verbose: false,
            author: None,
        };
        let mut config = Config::new();
        config.set_author("testuser".to_string());
        let author = Metadata::determine_author(&cli, &config);
        assert_eq!(author, "testuser");
    }

    #[test]
    fn test_determine_author_in_cli_and_config() {
        let cli = Cli {
            filename: "test".to_string(),
            overwrite: false,
            verbose: false,
            author: Option::from("cliuser".to_string()),
        };
        let mut config = Config::new();
        config.set_author("configuser".to_string());
        let author = Metadata::determine_author(&cli, &config);
        assert_eq!(author, "cliuser");
    }

    #[test]
    fn test_determine_author_whoami() {
        let cli = Cli {
            filename: "test".to_string(),
            overwrite: false,
            verbose: false,
            author: None,
        };
        let config = Config::new();
        let author = Metadata::determine_author(&cli, &config);
        assert_eq!(author, whoami::username());
    }

    #[test]
    fn test_metadata_from() {
        let cli = Cli {
            filename: "test".to_string(),
            overwrite: false,
            verbose: false,
            author: Option::from("testuser".to_string()),
        };
        let mut config = Config::new();
        config.extension = Some("md".to_string());
        config.set_author("configuser".to_string());
        let metadata = Metadata::from(&cli, &config);
        assert_eq!(metadata.filestem, "test");
        assert_eq!(metadata.author, "testuser");
        assert!(metadata.extension.is_some_and(|ext| ext == "md"));
    }
}
