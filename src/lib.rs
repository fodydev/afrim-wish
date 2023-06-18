mod config;
mod rstk_ext;

use clafrica::api;
use enigo::{Enigo, Key, KeyboardControllable};
use rstk::*;
use rstk_ext::*;
use std::collections::HashMap;

pub mod prelude {
    pub use crate::config::Config;
}

pub struct Wish {
    border: f64,
    label: Option<rstk::TkLabel>,
    prediction_frame: Option<TkFrame>,
    window: rstk::TkTopLevel,
    theme: HashMap<&'static str, Style>,
    suggestions: HashMap<String, String>,
    keyboard: Enigo,
}

impl Wish {
    pub fn init(config: config::Config) -> Self {
        let wish = if cfg!(debug_assertions) {
            rstk::trace_with("wish").unwrap()
        } else {
            rstk::start_wish().unwrap()
        };

        init_rstk_ext();

        let suggestions = config.extract_suggestions();

        let mut theme = HashMap::new();

        if let Some(theme_config) = config.theme {
            let header_frame_style = Style {
                name: "header.TFrame",
                background: theme_config.header.background.to_owned(),
                foreground: "".to_owned(),
                font_size: 0,
                font_family: "".to_owned(),
                font_weight: "".to_owned(),
            };
            header_frame_style.update();
            theme.insert("HFrame", header_frame_style);

            let header_label_style = Style {
                name: "header.TLabel",
                background: theme_config.header.background,
                foreground: theme_config.header.foreground,
                font_size: theme_config.header.font.size,
                font_family: theme_config.header.font.family,
                font_weight: theme_config.header.font.weight,
            };
            header_label_style.update();
            theme.insert("HLabel", header_label_style);

            let body_frame_style = Style {
                name: "body.TFrame",
                background: theme_config.body.background.to_owned(),
                foreground: "".to_owned(),
                font_size: 0,
                font_family: "".to_owned(),
                font_weight: "".to_owned(),
            };
            body_frame_style.update();
            theme.insert("BFrame", body_frame_style);

            let body_label_style = Style {
                name: "body.TLabel",
                background: theme_config.body.background,
                foreground: theme_config.body.foreground,
                font_size: theme_config.body.font.size,
                font_family: theme_config.body.font.family,
                font_weight: theme_config.body.font.weight,
            };
            body_label_style.update();
            theme.insert("BLabel", body_label_style);
        };

        Wish {
            window: wish,
            label: None,
            border: 0.0,
            prediction_frame: None,
            theme,
            suggestions,
            keyboard: Enigo::new(),
        }
    }

    pub fn build(&mut self) {
        self.window.title("Clafrica Wish");
        self.window.resizable(false, false);
        self.window.background("#dedddd");
        self.window.withdraw();
        self.window.border(false);
        self.window.topmost(true);
        self.window.deiconify();

        let header_frame = rstk::make_frame(&self.window);
        if let Some(v) = self.theme.get("HFrame") {
            header_frame.style(v);
        }

        let label = rstk::make_label(&header_frame);
        label.text("Type _exit_ to end the clafrica");
        if let Some(v) = self.theme.get("HLabel") {
            label.style(v);
        }
        label.pack().side(PackSide::Left).layout();

        header_frame.pack().fill(PackFill::X).layout();

        self.label = Some(label);
    }
}

impl api::Frontend for Wish {
    fn update_screen(&mut self, screen: (u64, u64)) {
        self.border = f64::sqrt((screen.0 * screen.0 + screen.1 * screen.1) as f64) / 100.0;
    }

    fn update_position(&mut self, position: (f64, f64)) {
        {
            let x = (position.0 + self.border) as u64;
            let y = (position.1 + self.border) as u64;
            self.window.position(x, y);
        }
    }

    fn update_text(&mut self, input: Vec<char>) {
        let input = input.into_iter().filter(|c| *c != '\0').collect::<String>();

        if input == "_exit_" {
            rstk::end_wish();
        }

        if let Some(label) = self.label.as_ref() {
            label.text(&input);
        }

        self.prediction_frame.as_ref().map(TkWidget::destroy);

        let prediction_frame = rstk::make_frame(&self.window);

        if input.len() > 1 {
            self.suggestions
                .iter()
                .filter(|(code, _text)| code.starts_with(input.as_str()))
                .enumerate()
                .for_each(|(i, (code, text))| {
                    if code.len() == input.len() {
                        (0..code.len()).for_each(|_| self.keyboard.key_click(Key::Backspace));
                        self.keyboard.key_sequence(text);
                        return;
                    }

                    let frame = rstk::make_frame(&prediction_frame);
                    if let Some(v) = self.theme.get("BFrame") {
                        frame.style(v);
                    }
                    frame.pack().fill(PackFill::X).layout();

                    let label = rstk::make_label(&frame);
                    label.text(&format!(
                        "{}. {text} ~{}",
                        i + 1,
                        code.chars().skip(input.len()).collect::<String>()
                    ));
                    if let Some(v) = self.theme.get("BLabel") {
                        label.style(v);
                    }
                    label.pack().side(PackSide::Left).layout();
                });
        }

        prediction_frame.pack().fill(PackFill::X).layout();

        self.prediction_frame = Some(prediction_frame);
    }
}
