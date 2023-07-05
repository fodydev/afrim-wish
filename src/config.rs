use serde::Deserialize;
use std::collections::HashMap;
use std::{error, fs, path::Path};
use toml::{self};

#[derive(Deserialize, Debug)]
pub struct Config {
    pub theme: Option<Theme>,
    pub core: Option<Core>,
    suggestions: HashMap<String, Data>,
}

#[derive(Deserialize, Debug)]
pub struct Core {
    pub page_size: usize,
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

#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
enum Data {
    Simple(String),
    File(DataFile),
}

#[derive(Deserialize, Debug, Clone)]
struct DataFile {
    path: String,
}

impl Config {
    pub fn from_file(filepath: &Path) -> Result<Self, Box<dyn error::Error>> {
        let content = fs::read_to_string(filepath)?;
        let mut config: Self = toml::from_str(&content)?;

        let config_path = filepath.parent().unwrap();

        let mut suggestions = HashMap::new();

        config.suggestions.iter().try_for_each(
            |(key, value)| -> Result<(), Box<dyn error::Error>> {
                match value {
                    Data::File(DataFile { path }) => {
                        let filepath = config_path.join(path);
                        let conf = Config::from_file(&filepath)?;
                        suggestions.extend(conf.suggestions);
                    }
                    Data::Simple(_) => {
                        suggestions.insert(key.to_owned(), value.clone());
                    }
                };
                Ok(())
            },
        )?;

        config.suggestions = suggestions;

        Ok(config)
    }

    pub fn extract_suggestions(&self) -> HashMap<String, String> {
        let mut suggestions = HashMap::new();

        self.suggestions.iter().for_each(|(k, v)| {
            match v {
                Data::Simple(value) => suggestions.insert(k.to_owned(), value.to_owned()),
                _ => None,
            };
        });

        suggestions
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

        let config = Config::from_file(Path::new("./data/sample2.toml"));
        assert!(config.is_ok());
    }
}
