#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use afrim::{run, Config as ClafricaConfig};
use afrim_wish::{Config as WishConfig, Wish};
use clap::Parser;

/// Afrim Wish CLI.
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

    let wish_conf = WishConfig::from_file(&args.config_file).map_err(|err| {
        Wish::raise_error("Problem parsing config file", &err);
    }).unwrap();

    let wish = Wish::from_config(wish_conf);

    let clafrica_conf = ClafricaConfig::from_file(&args.config_file).map_err(|err| {
        Wish::raise_error("Problem parsing config file", &err);
    }).unwrap();

    // End the program if check only.
    if args.check {
        Wish::kill();
    }

    if let Err(err) = run(clafrica_conf, wish) {
        Wish::raise_error("Application error", &err);
    }
}
