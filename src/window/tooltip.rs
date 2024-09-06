use super::config::Theme;
use afrim::frontend::Predicate;
use afrish::*;
use std::collections::HashMap;

#[derive(Clone, Default)]
pub struct ToolTip {
    themes: HashMap<&'static str, Style>,
    window: Option<afrish::TkTopLevel>,
    cursor_widget: Option<afrish::TkLabel>,
    predicates_widget: Option<afrish::TkLabel>,
    predicates: Vec<Predicate>,
    current_predicate_id: usize,
    page_size: usize,
    input: String,
    border: f64,
}

impl ToolTip {
    pub fn new(theme: Theme) -> Self {
        let mut themes = HashMap::new();

        let style = Style {
            name: "header.predicates.TLabel",
            background: theme.header.background,
            foreground: theme.header.foreground,
            font_size: theme.header.font.size,
            font_family: theme.header.font.family,
            font_weight: theme.header.font.weight,
        };
        themes.insert("PHLabel", style);

        let style = Style {
            name: "body.predicates.TLabel",
            background: theme.body.background,
            foreground: theme.body.foreground,
            font_size: theme.body.font.size,
            font_family: theme.body.font.family.to_owned(),
            font_weight: theme.body.font.weight.to_owned(),
        };
        themes.insert("PBLabel", style);

        Self {
            themes,
            ..Default::default()
        }
    }

    fn build_theme(&self) {
        self.themes.iter().for_each(|(_, style)| style.update());
    }

    fn build_window(&mut self) {
        let window = self.window.as_ref().unwrap();
        window.resizable(false, false);
        window.background("#dedddd");
        window.withdraw();
        window.border(false);
        window.topmost(true);
        window.deiconify();

        // Cursor
        let cursor_widget = afrish::make_label(window);
        cursor_widget.text("Afrim is ready for input!");
        cursor_widget.style(&self.themes["PHLabel"]);
        cursor_widget.pack().fill(PackFill::X).layout();
        self.cursor_widget = Some(cursor_widget);

        // Predication
        let predicates_widget = afrish::make_label(window);
        predicates_widget.style(&self.themes["PBLabel"]);
        predicates_widget.pack().fill(PackFill::X).layout();
        self.predicates_widget = Some(predicates_widget);
    }

    pub fn build(&mut self, window: afrish::TkTopLevel) {
        self.window = Some(window);
        self.build_theme();
        self.build_window();
    }

    pub fn update_screen(&mut self, screen: (u64, u64)) {
        self.border = f64::sqrt((screen.0 * screen.0 + screen.1 * screen.1) as f64) / 100.0;
    }

    pub fn update_position(&mut self, position: (f64, f64)) {
        let x = (position.0 + self.border) as u64;
        let y = (position.1 + self.border) as u64;
        self.window.as_ref().unwrap().position(x, y);
    }

    pub fn set_input_text(&mut self, text: String) {
        self.input = text;
    }

    pub fn set_page_size(&mut self, size: usize) {
        self.page_size = size;
    }

    pub fn add_predicate(&mut self, predicate: Predicate) {
        predicate
            .texts
            .iter()
            .filter(|text| !text.is_empty())
            .for_each(|text| {
                let mut predicate = predicate.clone();
                predicate.texts = vec![text.to_owned()];

                self.predicates.push(predicate);
            });
    }

    pub fn clear(&mut self) {
        self.predicates.clear();
        self.current_predicate_id = 0;
        self.input = String::default();
    }

    pub fn select_previous_predicate(&mut self) {
        if self.predicates.is_empty() {
            return;
        }

        self.current_predicate_id =
            (self.current_predicate_id + self.predicates.len() - 1) % self.predicates.len();
        self.update();
    }

    pub fn select_next_predicate(&mut self) {
        if self.predicates.is_empty() {
            return;
        }

        self.current_predicate_id = (self.current_predicate_id + 1) % self.predicates.len();
        self.update();
    }

    pub fn get_selected_predicate(&self) -> Option<&Predicate> {
        self.predicates.get(self.current_predicate_id)
    }

    pub fn update(&self) {
        let page_size = std::cmp::min(self.page_size, self.predicates.len());
        let texts: Vec<String> = self
            .predicates
            .iter()
            .enumerate()
            .chain(self.predicates.iter().enumerate())
            .skip(self.current_predicate_id)
            .take(page_size)
            .map(|(i, predicate)| {
                format!(
                    "{}. {} ~{}",
                    i + 1,
                    predicate.texts[0],
                    predicate.remaining_code
                )
            })
            .collect();

        self.cursor_widget.as_ref().unwrap().text(&self.input);
        self.predicates_widget
            .as_ref()
            .unwrap()
            .text(&texts.join("\n"));
    }
}
