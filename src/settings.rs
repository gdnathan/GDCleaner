use clap::Parser;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::collections::HashMap;

const DEFAULT_CONFIG: &str = include_str!("../config.toml");

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Params {
    pub identifiers: Vec<String>,
    pub targets: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Config {
    pub path: PathBuf,
    pub force: bool,
    pub verbose: bool,
    /// Identify language name from a file name
    pub lang_identifier: HashMap<String, String>,
    /// Identify language name from a target name
    pub lang_target: HashMap<String, String>,
    pub skip_size: bool
}

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path from which to run the program
    #[arg(default_value_t = String::from("."))]
    path: String,
    
    /// Delete build folders without asking
    #[arg(short, long, default_value_t = false)]
    force: bool,

    /// More logs
    #[arg(short, long, default_value_t = false)]
    verbose: bool,

    /// Only run the program for this specific target language
    #[arg(long)]
    only: Option<String>,

    /// Skip size calculation. This will clearly boost speed
    #[arg(long, short, default_value_t = false)]
    pub skip_size: bool
}

pub fn generate_config() -> Config {

    let mut langs: HashMap<String, Params> = toml::from_str(DEFAULT_CONFIG).unwrap();

    let args = Args::parse();

    if let Some(only) = args.only {
        langs.retain(|k, _| *k == only);
        if langs.is_empty() {
            eprintln!("Language {} not found in config.toml (specified by --only)", only);
            std::process::exit(1);
        }
    }

    let path = PathBuf::from(&args.path).canonicalize().unwrap();

    let mut lang_identifier = HashMap::<String, String>::new();
    let mut lang_target = HashMap::<String, String>::new();

    langs.into_iter().for_each(|(name, params)| {
        for identifier in params.identifiers {
            lang_identifier.insert(identifier, name.clone());
        }
        for target in params.targets {
            lang_target.insert(target, name.clone());
        }
    });

    Config {
        path,
        force: args.force,
        verbose: args.verbose,
        lang_identifier,
        lang_target,
        skip_size: args.skip_size
    }
}
