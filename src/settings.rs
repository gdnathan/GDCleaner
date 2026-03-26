use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Lang {
    pub name: String,
    pub identifiers: Vec<String>,
    pub targets: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct Langs {
    pub lang: Vec<Lang>,
}

#[derive(Debug)]
pub struct Config {
    pub force: bool,
    pub verbose: bool,
    pub langs: Vec<Lang>,
}

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Delete build folders without asking
    #[arg(short, long, default_value_t = false)]
    force: bool,

    /// More logs
    #[arg(short, long, default_value_t = false)]
    verbose: bool,

    /// Only run the program for this specific target language
    #[arg(long)]
    only: Option<String>,
}

pub fn generate_config() -> Config {
    let config_str = std::fs::read_to_string("config.toml").unwrap();
    let mut langs: Vec<Lang> = toml::from_str::<Langs>(&config_str).unwrap().lang;

    let args = Args::parse();

    if let Some(only) = args.only {
        langs.retain(|l| l.name == only);
        if langs.is_empty() {
            eprintln!("Language {} not found in config.toml (specified by --only)", only);
            std::process::exit(1);
        }
    }

    Config {
        force: args.force,
        verbose: args.verbose,
        langs
    }
}
