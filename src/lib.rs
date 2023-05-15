mod config;
mod rstk_ext;

use clafrica::api;
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
}

impl Wish {
    pub fn init(config: config::Config) -> Self {
        let wish = rstk::start_wish().unwrap();

        let mut theme = HashMap::new();

        let header_frame_style = Style {
            name: "header.TFrame",
            background: config.theme.header.background.to_owned(),
            foreground: "".to_owned(),
            font_size: 0,
            font_family: "".to_owned(),
            font_weight: "".to_owned(),
        };
        header_frame_style.update();
        theme.insert("HFrame", header_frame_style);

        let header_label_style = Style {
            name: "header.TLabel",
            background: config.theme.header.background,
            foreground: config.theme.header.foreground,
            font_size: config.theme.header.font.size,
            font_family: config.theme.header.font.family,
            font_weight: config.theme.header.font.weight,
        };
        header_label_style.update();
        theme.insert("HLabel", header_label_style);

        let body_frame_style = Style {
            name: "body.TFrame",
            background: config.theme.body.background.to_owned(),
            foreground: "".to_owned(),
            font_size: 0,
            font_family: "".to_owned(),
            font_weight: "".to_owned(),
        };
        body_frame_style.update();
        theme.insert("BFrame", body_frame_style);

        let body_label_style = Style {
            name: "body.TLabel",
            background: config.theme.body.background,
            foreground: config.theme.body.foreground,
            font_size: config.theme.body.font.size,
            font_family: config.theme.body.font.family,
            font_weight: config.theme.body.font.weight,
        };
        body_label_style.update();
        theme.insert("BLabel", body_label_style);

        Wish {
            window: wish,
            label: None,
            border: 0.0,
            prediction_frame: None,
            theme,
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
        header_frame.style(self.theme.get("HFrame").unwrap());

        let label = rstk::make_label(&header_frame);
        label.text("Type _exit_ to end the clafrica");
        label.style(self.theme.get("HLabel").unwrap());
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

    fn update_text(&mut self, text: Vec<char>) {
        let text = text
            .into_iter()
            .map(|c| c.to_string())
            .collect::<Vec<_>>()
            .join("");

        if text == "_exit_" {
            rstk::end_wish();
        }

        if let Some(label) = self.label.as_ref() {
            label.text(&text);
        }

        let predictions = ["Wish"];

        self.prediction_frame.as_ref().map(TkWidget::destroy);

        let prediction_frame = rstk::make_frame(&self.window);
        predictions.iter().enumerate().for_each(|(i, e)| {
            let frame = rstk::make_frame(&prediction_frame);
            frame.style(self.theme.get("BFrame").unwrap());
            frame.pack().fill(PackFill::X).layout();

            let label = rstk::make_label(&frame);
            label.text(&format!("{}. {e}", i + 1));
            label.style(self.theme.get("BLabel").unwrap());
            label.pack().side(PackSide::Left).layout();
        });
        prediction_frame.pack().fill(PackFill::X).layout();

        self.prediction_frame = Some(prediction_frame);
    }
}
