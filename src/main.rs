#![windows_subsystem = "windows"]

use clafrica::{run, Config};
use clafrica_wish::Wish;
use std::{env, process};

fn main() {
    let conf = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    let frontend = Wish::build(conf.buffer_size);

    if let Err(e) = run(conf, frontend) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
