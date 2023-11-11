#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use afrim::{run, Config as ClafricaConfig};
use afrim_wish::{Config as WishConfig, Wish};
use clap::Parser;
use std::process;

/// Afrim CLI.
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the configuration file.
    config_file: std::path::PathBuf,

    /// Only verify if the configuration file is valid.
    #[arg(long, action)]
    check: bool,
}

fn main() {
    let args = Args::parse();

    let clafrica_conf = ClafricaConfig::from_file(&args.config_file).unwrap_or_else(|err| {
        eprintln!("Problem parsing config file: {err}");
        process::exit(1);
    });

    let wish_conf = WishConfig::from_file(&args.config_file).unwrap_or_else(|err| {
        eprintln!("Problem parsing config file: {err}");
        process::exit(1);
    });

    let mut frontend = Wish::init(wish_conf);
    frontend.build();

    if let Err(e) = run(clafrica_conf, frontend) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
