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
