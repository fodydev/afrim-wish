#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use afrim::{run, Config as ClafricaConfig};
use afrim_wish::{Config as WishConfig, Wish};
use clap::Parser;
use std::process;
use std::thread;

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

    let wish_conf = WishConfig::from_file(&args.config_file).unwrap_or_else(|err| {
        eprintln!("Problem parsing config file: {err}");
        process::exit(1);
    });

    let mut frontend = Wish::init(wish_conf);
    frontend.build();

    // We start the backend
    {
        let frontend = frontend.clone();

        thread::spawn(move || {
            let clafrica_conf =
                ClafricaConfig::from_file(&args.config_file).unwrap_or_else(|err| {
                    frontend.raise_error("Problem parsing config file", &err.to_string());
                    process::exit(1);
                });

            if let Err(e) = run(clafrica_conf, frontend.clone()) {
                frontend.raise_error("Application error", &e.to_string());
                process::exit(1);
            }
        });
    }

    // We start listening gui events
    frontend.listen();
}
