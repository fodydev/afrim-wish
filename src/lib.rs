mod config;
mod rstk_ext;

use afrim::frontend::Frontend;
use rstk::*;
use rstk_ext::*;
use std::collections::HashMap;

pub use crate::config::Config;

pub struct Wish {
    border: f64,
    window: rstk::TkTopLevel,
    themes: HashMap<&'static str, Style>,
    predicates: Vec<(String, String, String)>,
    page_size: usize,
    current_predicate_id: usize,
    input: String,
    cursor_widget: rstk::TkLabel,
    predicates_widget: rstk::TkLabel,
}

impl Wish {
    pub fn init(config: config::Config) -> Self {
        let wish = if cfg!(debug_assertions) {
            rstk::trace_with("wish").unwrap()
        } else {
            rstk::start_wish().unwrap()
        };

        init_rstk_ext();

        let mut themes = HashMap::new();

        if let Some(theme_config) = config.theme {
            let style = Style {
                name: "header.predicates.TLabel",
                background: theme_config.header.background,
                foreground: theme_config.header.foreground,
                font_size: theme_config.header.font.size,
                font_family: theme_config.header.font.family,
                font_weight: theme_config.header.font.weight,
            };
            style.update();
            themes.insert("PHLabel", style);

            let style = Style {
                name: "body.predicates.TLabel",
                background: theme_config.body.background,
                foreground: theme_config.body.foreground,
                font_size: theme_config.body.font.size,
                font_family: theme_config.body.font.family.to_owned(),
                font_weight: theme_config.body.font.weight,
            };
            style.update();
            themes.insert("PBLabel", style);
        };

        Wish {
            predicates_widget: rstk::make_label(&wish),
            cursor_widget: rstk::make_label(&wish),
            window: wish,
            border: 0.0,
            themes,
            predicates: Vec::new(),
            page_size: 10,
            current_predicate_id: 0,
            input: "".to_owned(),
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

        // Cursor
        self.cursor_widget = rstk::make_label(&self.window);
        self.cursor_widget.text("Type _exit_ to end the clafrica");
        self.cursor_widget.style(&self.themes["PHLabel"]);
        self.cursor_widget.pack().fill(PackFill::X).layout();

        // Predication
        self.predicates_widget = rstk::make_label(&self.window);
        self.predicates_widget.style(&self.themes["PBLabel"]);
        self.predicates_widget.pack().fill(PackFill::X).layout();
    }
}

impl Frontend for Wish {
    fn update_screen(&mut self, screen: (u64, u64)) {
        self.border = f64::sqrt((screen.0 * screen.0 + screen.1 * screen.1) as f64) / 100.0;
    }

    fn update_position(&mut self, position: (f64, f64)) {
        let x = (position.0 + self.border) as u64;
        let y = (position.1 + self.border) as u64;
        self.window.position(x, y);
    }

    fn set_input(&mut self, text: &str) {
        self.input = text.to_owned();
    }

    fn set_page_size(&mut self, size: usize) {
        self.page_size = size;
    }

    fn add_predicate(&mut self, code: &str, remaining_code: &str, text: &str) {
        self.predicates
            .push((code.to_owned(), remaining_code.to_owned(), text.to_owned()));
    }

    fn clear_predicates(&mut self) {
        self.predicates.clear();
        self.current_predicate_id = 0;
    }

    fn previous_predicate(&mut self) {
        if self.predicates.is_empty() {
            return;
        }

        self.current_predicate_id =
            (self.current_predicate_id + self.predicates.len() - 1) % self.predicates.len();
        self.display();
    }

    fn next_predicate(&mut self) {
        if self.predicates.is_empty() {
            return;
        }

        self.current_predicate_id = (self.current_predicate_id + 1) % self.predicates.len();
        self.display();
    }

    fn get_selected_predicate(&self) -> Option<&(String, String, String)> {
        self.predicates.get(self.current_predicate_id)
    }

    fn display(&self) {
        if self.input == "_exit_" {
            rstk::end_wish();
        }

        let page_size = std::cmp::min(self.page_size, self.predicates.len());
        let texts: Vec<String> = self
            .predicates
            .iter()
            .enumerate()
            .chain(self.predicates.iter().enumerate())
            .skip(self.current_predicate_id)
            .take(page_size)
            .map(|(i, (_code, remaining_code, text))| {
                format!("{}. {text} ~{remaining_code}", i + 1,)
            })
            .collect();

        self.cursor_widget.text(&self.input);
        self.predicates_widget.text(&texts.join("\n"));
    }
}
