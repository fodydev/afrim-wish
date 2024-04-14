mod config;
mod rstk_ext;

use afrim::frontend::Frontend;
use rstk::*;
use rstk_ext::*;
use std::collections::HashMap;

pub use crate::config::Config;

// Ratio to easily adjust the dimension of the gui
const GUI_RATIO: f64 = 0.8;

#[derive(Clone)]
pub struct Wish {
    border: f64,
    predicates_window: rstk::TkTopLevel,
    window: rstk::TkTopLevel,
    themes: HashMap<&'static str, Style>,
    predicates: Vec<(String, String, String)>,
    page_size: usize,
    current_predicate_id: usize,
    input: String,
    cursor_widget: rstk::TkLabel,
    predicates_widget: rstk::TkLabel,
    config: config::Config,
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

        let theme_config = config.theme.clone().unwrap_or_default();
        // Predicates
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
            font_weight: theme_config.body.font.weight.to_owned(),
        };
        style.update();
        themes.insert("PBLabel", style);

        // Toolkit
        let font_family = "Charis-SIL";
        let style = Style {
            name: "toolkit.TFrame",
            background: "#1e1e1e".to_owned(),
            ..Default::default()
        };
        style.update();
        themes.insert("TFrame", style);

        let style = Style {
            name: "label.toolkit.TLabel",
            background: "#1e1e1e".to_owned(),
            foreground: "#ffffff".to_owned(),
            font_size: (12.0 * GUI_RATIO) as u64,
            font_family: font_family.to_string(),
            ..Default::default()
        };
        style.update();
        themes.insert("TLabel", style);

        let style = Style {
            name: "button.toolkit.TButton",
            background: "#ffffff".to_owned(),
            foreground: "#1e1e1e".to_owned(),
            font_size: (12.0 * GUI_RATIO) as u64,
            font_family: font_family.to_string(),
            ..Default::default()
        };
        style.update();
        themes.insert("TButton", style);

        let style = Style {
            name: "exit.toolkit.TButton",
            background: "#e03131".to_owned(),
            foreground: "#1e1e1e".to_owned(),
            font_size: (12.0 * GUI_RATIO) as u64,
            font_family: font_family.to_string(),
            font_weight: "bold".to_owned(),
        };
        style.update();
        themes.insert("TEButton", style);

        let style = Style {
            name: "iconify.toolkit.TButton",
            background: "#1971c2".to_owned(),
            foreground: "#1e1e1e".to_owned(),
            font_size: (12.0 * GUI_RATIO) as u64,
            font_family: font_family.to_string(),
            font_weight: "bold".to_owned(),
        };
        style.update();
        themes.insert("TIButton", style);

        let style = Style {
            name: "toolkit.TNotebook",
            background: "#1e1e1e".to_owned(),
            ..Default::default()
        };
        style.update();
        themes.insert("TNotebook", style);

        Style {
            name: "toolkit.TNotebook.Tab",
            background: "#ffffff".to_owned(),
            foreground: "#1e1e1e".to_owned(),
            font_size: (12.0 * GUI_RATIO) as u64,
            font_family: font_family.to_string(),
            ..Default::default()
        }
        .update();

        Wish {
            predicates_widget: rstk::make_label(&wish),
            cursor_widget: rstk::make_label(&wish),
            predicates_window: rstk::make_toplevel(&wish),
            window: wish,
            border: 0.0,
            themes,
            predicates: Vec::new(),
            page_size: 10,
            current_predicate_id: 0,
            input: "".to_owned(),
            config,
        }
    }

    pub fn raise_error(&self, message: &str, detail: &str) {
        self.predicates_window.withdraw();
        rstk::message_box()
            .parent(&self.window)
            .icon(IconImage::Error)
            .title("Unexpected Error")
            .message(message)
            .detail(detail)
            .show();
        rstk::end_wish();
    }

    fn build_predicates_window(&mut self) {
        self.predicates_window.resizable(false, false);
        self.predicates_window.background("#dedddd");
        self.predicates_window.withdraw();
        self.predicates_window.border(false);
        self.predicates_window.topmost(true);
        self.predicates_window.deiconify();

        // Cursor
        self.cursor_widget = rstk::make_label(&self.predicates_window);
        self.cursor_widget.text("Type _exit_ to end the clafrica");
        self.cursor_widget.style(&self.themes["PHLabel"]);
        self.cursor_widget.pack().fill(PackFill::X).layout();

        // Predication
        self.predicates_widget = rstk::make_label(&self.predicates_window);
        self.predicates_widget.style(&self.themes["PBLabel"]);
        self.predicates_widget.pack().fill(PackFill::X).layout();
    }

    fn build_main_window(&self) {
        self.window.title("Afrim Wish");
        self.window.resizable(false, false);
        self.window.background("#1e1e1e");
        self.window.geometry(
            (480.0 * GUI_RATIO) as u64,
            (250.0 * GUI_RATIO) as u64,
            -1,
            -1,
        );

        // Header
        let frame = rstk::make_frame(&self.window);
        frame.style(&self.themes["TFrame"]);
        // Header label
        let label = rstk::make_label(&frame);
        label.text("AFRIM Toolkit");
        label.style(&self.themes["TLabel"]);
        label.pack().side(PackSide::Left).layout();
        // Header iconify button
        let button = rstk::make_button(&frame);
        button.text("x");
        button.width((4.0 * GUI_RATIO) as i64);
        button.style(&self.themes["TEButton"]);
        button.command(rstk::end_wish);
        button
            .pack()
            .side(PackSide::Right)
            .padx((5.0 * GUI_RATIO) as u64)
            .layout();
        // Header exit button
        let button = rstk::make_button(&frame);
        button.text("-");
        {
            let window = self.window.clone();
            button.command(move || window.iconify());
        }
        button.width((4.0 * GUI_RATIO) as i64);
        button.style(&self.themes["TIButton"]);
        button
            .pack()
            .side(PackSide::Right)
            .padx((5.0 * GUI_RATIO) as u64)
            .layout();
        // We build the header
        frame
            .pack()
            .fill(PackFill::X)
            .padx((20.0 * GUI_RATIO) as u64)
            .pady((15.0 * GUI_RATIO) as u64)
            .layout();

        // Separator
        rstk::make_frame(&self.window)
            .pack()
            .fill(PackFill::X)
            .padx((30.0 * GUI_RATIO) as u64)
            .layout();
        let frame = rstk::make_frame(&self.window);
        frame.style(&self.themes["TFrame"]);
        frame
            .pack()
            .fill(PackFill::X)
            .pady((10.0 * GUI_RATIO) as u64)
            .layout();

        // Body
        let notebook = rstk::make_notebook(&self.window);
        notebook.style(&self.themes["TNotebook"]);

        // Page builder
        macro_rules! make_page {
            ( $tabname: expr, $($fieldname: expr => $fieldvalue: expr => $see_more: stmt)*) => {
                let frame = rstk::make_frame(&self.window);
                frame.style(&self.themes["TFrame"]);

                $(
                    let subframe = rstk::make_frame(&frame);
                    subframe.style(&self.themes["TFrame"]);
                    let label = rstk::make_label(&subframe);
                    label.text($fieldname);
                    label.style(&self.themes["TLabel"]);
                    label.pack().side(PackSide::Left).layout();
                    let button = rstk::make_button(&subframe);
                    button.text($fieldvalue);
                    button.width((25.0 * GUI_RATIO) as i64);
                    button.style(&self.themes["TButton"]);
                    let cmd = {$see_more};
                    button.command(cmd);
                    button.pack().side(PackSide::Right).layout();
                    // We build the field
                    subframe
                        .pack()
                        .fill(PackFill::X)
                        .pady((2.0 * GUI_RATIO) as u64)
                        .layout();
                )*

                notebook.add(&frame, $tabname)
            };
        }

        // Details page
        make_page!(
            "Details",
            "IME:" => &self.config.info.input_method => {
                let window = self.window.clone();
                let config_name = self.config.info.name.to_owned();
                let config_maintainors = self.config.info.maintainors.join(", ");
                let config_homepage = self.config.info.homepage.clone();
                move || {
                    rstk::message_box()
                        .parent(&window)
                        .icon(IconImage::Information)
                        .title("Configuration file")
                        .message(&config_name)
                        .detail(&format!(
                            "{}\n\nby {}",
                            &config_homepage,
                            &config_maintainors,
                        ))
                        .show();
                }
            }
            "Auto Commit:" => &self.config.core.as_ref().unwrap_or_default().auto_commit.to_string() => || ()
            "Buffer Size:" => &self.config.core.as_ref().unwrap_or_default().buffer_size.to_string() => || ()
        );

        // Help page
        make_page!(
            "Help",
            "Keyboard shortcuts:" => "open" => {
                let window = self.window.clone();

                move || {
                    rstk::message_box()
                        .parent(&window)
                        .icon(IconImage::Information)
                        .title("Keyboard shortcuts")
                        .message("Keyboard shortcuts")
                        .detail("\
                            Command -> Shortcuts\n\n\
                            Pause / Resume -> CtrlLeft + CtrlRight\n\n\
                            Clear -> Escape / Space\n\n\
                            Select Next Predicate -> Ctrl + ShiftLeft\n\n\
                            Select Previous Predicate -> Ctrl + ShiftRight\n\n\
                            Commit Selected Predicate -> Ctrl + Space\
                        ")
                        .show();
                }
            }
            "About Afrim Wish:" => "open" => {
                let window = self.window.clone();

                move || {
                    rstk::message_box()
                        .parent(&window)
                        .icon(IconImage::Information)
                        .title("About")
                        .message(env!("CARGO_PKG_NAME"))
                        .detail(&format!(
                            "\
                            version: {}\n\n\
                            by {}\n\n\
                            {}\n\n\
                            {}\n\n\
                            This program comes with absolutely no warranty.\n\
                            See the {} license for more details.\
                            ",
                            env!("CARGO_PKG_VERSION"),
                            env!("CARGO_PKG_AUTHORS"),
                            env!("CARGO_PKG_DESCRIPTION"),
                            env!("CARGO_PKG_REPOSITORY"),
                            env!("CARGO_PKG_LICENSE")
                        ))
                        .show();
                }
            }
        );

        // We build the notebook
        notebook
            .pack()
            .fill(PackFill::X)
            .padx((20.0 * GUI_RATIO) as u64)
            .layout();
    }

    pub fn build(&mut self) {
        self.build_predicates_window();
        self.build_main_window();
    }

    pub fn listen(&self) {
        rstk::mainloop();
    }

    pub fn destroy(&self) {
        rstk::end_wish();
    }
}

impl Frontend for Wish {
    fn update_screen(&mut self, screen: (u64, u64)) {
        self.border = f64::sqrt((screen.0 * screen.0 + screen.1 * screen.1) as f64) / 100.0;
    }

    fn update_position(&mut self, position: (f64, f64)) {
        let x = (position.0 + self.border) as u64;
        let y = (position.1 + self.border) as u64;
        self.predicates_window.position(x, y);
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

#[cfg(test)]
mod tests {
    use crate::{Config, Wish};
    use afrim::frontend::Frontend;
    use std::path::Path;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_api() {
        let config = Config::from_file(Path::new("data/full_sample.toml")).unwrap();
        let mut afrim_wish = Wish::init(config);
        afrim_wish.build();

        // Test without data.
        afrim_wish.clear_predicates();
        afrim_wish.next_predicate();
        afrim_wish.previous_predicate();
        assert!(afrim_wish.get_selected_predicate().is_none());
        afrim_wish.display();

        // Test the adding of predicates.
        afrim_wish.set_page_size(3);
        afrim_wish.set_input("Test started!");
        afrim_wish.add_predicate("test", "123", "ok");
        afrim_wish.add_predicate("test1", "23", "ok");
        afrim_wish.add_predicate("test12", "1", "ok");
        afrim_wish.add_predicate("test123", "", "ok");
        afrim_wish.add_predicate("test1234", "", "");
        afrim_wish.display();

        // Test the geometry.
        (0..100).for_each(|i| {
            if i % 10 != 0 {
                return;
            };
            let i = i as f64;
            afrim_wish.update_position((i, i));
            thread::sleep(Duration::from_millis(100));
        });

        // Test the navigation.
        afrim_wish.previous_predicate();
        assert_eq!(
            afrim_wish.get_selected_predicate(),
            Some(&("test1234".to_owned(), "".to_owned(), "".to_owned()))
        );
        afrim_wish.next_predicate();
        assert_eq!(
            afrim_wish.get_selected_predicate(),
            Some(&("test".to_owned(), "123".to_owned(), "ok".to_owned()))
        );
        afrim_wish.display();
    }
}
