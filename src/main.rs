#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use clafrica::{prelude::Config as ClafricaConfig, run};
use clafrica_wish::{prelude::Config as WishConfig, Wish};
use std::{env, path::Path, process};

fn main() {
    let filename = env::args().nth(1).unwrap_or_else(|| {
        eprintln!("Configuration file required");
        process::exit(1);
    });

    let clafrica_conf = ClafricaConfig::from_file(Path::new(&filename)).unwrap_or_else(|err| {
        eprintln!("Problem parsing config file: {err}");
        process::exit(1);
    });

    let wish_conf = WishConfig::from_file(Path::new(&filename)).unwrap_or_else(|err| {
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
