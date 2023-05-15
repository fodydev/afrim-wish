use serde::Deserialize;
use std::{error, fs};
use toml::{self};

#[derive(Deserialize, Debug)]
pub struct Config {
    pub theme: Theme,
}

#[derive(Deserialize, Debug)]
pub struct Theme {
    pub header: SectionTheme,
    pub body: SectionTheme,
}

#[derive(Deserialize, Debug)]
pub struct SectionTheme {
    pub background: String,
    pub foreground: String,
    pub font: ThemeFont,
}

#[derive(Deserialize, Debug)]
pub struct ThemeFont {
    pub family: String,
    pub size: u64,
    pub weight: String,
}

impl Config {
    pub fn from_file(filename: &str) -> Result<Self, Box<dyn error::Error>> {
        let data = fs::read_to_string(filename)?;
        let config: Self = toml::from_str(&data)?;
        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn from_file() {
        use crate::config::Config;

        let config = Config::from_file("./data/sample.toml");
        assert!(config.is_ok());
    }
}
