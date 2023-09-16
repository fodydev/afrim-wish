use serde::Deserialize;
use std::{error, fs, path::Path};
use toml::{self};

#[derive(Deserialize, Debug)]
pub struct Config {
    pub theme: Option<Theme>,
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
    pub fn from_file(filepath: &Path) -> Result<Self, Box<dyn error::Error>> {
        let content = fs::read_to_string(filepath)?;
        let config: Self = toml::from_str(&content)?;

        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn from_file() {
        use crate::config::Config;
        use std::path::Path;

        let config = Config::from_file(Path::new("./data/sample.toml"));
        assert!(config.is_ok());

        let config = Config::from_file(Path::new("./data/blank_sample.toml"));
        assert!(config.is_ok());
    }
}
