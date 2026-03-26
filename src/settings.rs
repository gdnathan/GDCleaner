use clap::Parser;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::collections::HashMap;
use config::Config as ConfigRs;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Params {
    pub identifiers: Vec<String>,
    pub targets: Vec<String>,
}

#[derive(Debug)]
pub struct Config {
    pub path: PathBuf,
    pub force: bool,
    pub verbose: bool,
    pub langs: HashMap<String, Params>,
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
}

pub fn generate_config() -> Config {
    let mut langs = ConfigRs::builder()
        .add_source(config::File::with_name("config.toml"))
        .build()
        .unwrap()
        .try_deserialize::<HashMap<String, Params>>()
        .unwrap();

    let args = Args::parse();

    if let Some(only) = args.only {
        langs.retain(|k, _| *k == only);
        if langs.is_empty() {
            eprintln!("Language {} not found in config.toml (specified by --only)", only);
            std::process::exit(1);
        }
    }

    let path = PathBuf::from(&args.path).canonicalize().unwrap();

    Config {
        path,
        force: args.force,
        verbose: args.verbose,
        langs
    }
}
