#![windows_subsystem = "windows"]

use clafrica::{run, Config as ClafricaConfig};
use clafrica_wish::{prelude::Config as WishConfig, Wish};
use std::{env, process};

fn main() {
    let clafrica_conf = ClafricaConfig::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    let wish_conf = WishConfig::from_file("./data/wish.toml").unwrap_or_else(|err| {
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
