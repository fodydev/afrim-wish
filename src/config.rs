use anyhow::{Context, Result};
use serde::Deserialize;
use std::{fs, path::Path};
use toml::{self};

#[derive(Clone, Deserialize, Debug, Default)]
pub struct Config {
    pub theme: Option<Theme>,
    pub core: Option<Core>,
    pub info: Option<Info>,
}

#[derive(Clone, Deserialize, Debug)]
pub struct Core {
    pub buffer_size: i8,
    pub auto_commit: bool,
}

#[derive(Clone, Deserialize, Debug)]
pub struct Info {
    pub name: String,
    pub maintainors: Vec<String>,
    pub input_method: String,
    pub homepage: String,
}

#[derive(Clone, Deserialize, Debug)]
pub struct Theme {
    pub header: SectionTheme,
    pub body: SectionTheme,
}

#[derive(Clone, Deserialize, Debug)]
pub struct SectionTheme {
    pub background: String,
    pub foreground: String,
    pub font: ThemeFont,
}

#[derive(Clone, Deserialize, Debug)]
pub struct ThemeFont {
    pub family: String,
    pub size: u64,
    pub weight: String,
}

impl Default for &Core {
    fn default() -> Self {
        &Core {
            buffer_size: 64,
            auto_commit: false,
        }
    }
}

impl Default for Core {
    fn default() -> Self {
        Core {
            buffer_size: 64,
            auto_commit: false,
        }
    }
}

impl Default for Theme {
    fn default() -> Self {
        let font = ThemeFont {
            family: "Charis-SIL".to_owned(),
            size: 10,
            weight: "bold".to_owned(),
        };
        let header = SectionTheme {
            background: "#252320".to_owned(),
            foreground: "#dedddd".to_owned(),
            font: font.clone(),
        };
        let body = SectionTheme {
            background: "#dedddd".to_owned(),
            foreground: "#252320".to_owned(),
            font,
        };

        Self { header, body }
    }
}

impl Default for Info {
    fn default() -> Self {
        Self {
            name: "Unknown".to_owned(),
            input_method: "Unknow".to_owned(),
            homepage: "Unknow".to_owned(),
            maintainors: vec!["Unknow".to_owned()],
        }
    }
}

impl Config {
    pub fn from_file(filepath: &Path) -> Result<Self> {
        let content = fs::read_to_string(filepath)
            .with_context(|| format!("Couldn't open file {filepath:?}"))?;
        let config: Self = toml::from_str(&content)
            .with_context(|| format!("Failed to parse configuration file {filepath:?}"))?;

        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn from_file() {
        use crate::config::Config;
        use std::path::Path;

        let config = Config::from_file(Path::new("./data/blank_sample.toml"));
        assert!(config.is_ok());

        // Load default core and theme.
        let config = config.unwrap();
        config.core.unwrap_or_default();
        config.theme.unwrap_or_default();
        config.info.unwrap_or_default();

        let config = Config::from_file(Path::new("./data/full_sample.toml"));
        assert!(config.is_ok());

        let config = Config::from_file(Path::new("./data/sample.toml"));
        assert!(config.is_ok());
    }
}
