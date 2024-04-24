use super::config::Config;
use super::rstk_ext::*;
use rstk::*;
use std::collections::HashMap;

// Ratio to easily adjust the dimension of the gui
const GUI_RATIO: f64 = 0.8;

#[derive(Clone, Default)]
pub struct ToolKit {
    themes: HashMap<&'static str, Style>,
    window: Option<rstk::TkTopLevel>,
    config: Config,
}

impl ToolKit {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            ..Default::default()
        }
    }

    fn build_theme(&mut self) {
        let font_family = "Charis-SIL";

        let style = Style {
            name: "toolkit.TFrame",
            background: "#1e1e1e".to_owned(),
            ..Default::default()
        };
        self.themes.insert("TFrame", style);

        let style = Style {
            name: "label.toolkit.TLabel",
            background: "#1e1e1e".to_owned(),
            foreground: "#ffffff".to_owned(),
            font_size: (12.0 * GUI_RATIO) as u64,
            font_family: font_family.to_string(),
            ..Default::default()
        };
        self.themes.insert("TLabel", style);

        let style = Style {
            name: "button.toolkit.TButton",
            background: "#ffffff".to_owned(),
            foreground: "#1e1e1e".to_owned(),
            font_size: (12.0 * GUI_RATIO) as u64,
            font_family: font_family.to_string(),
            ..Default::default()
        };
        self.themes.insert("TButton", style);

        let style = Style {
            name: "exit.toolkit.TButton",
            background: "#e03131".to_owned(),
            foreground: "#1e1e1e".to_owned(),
            font_size: (12.0 * GUI_RATIO) as u64,
            font_family: font_family.to_string(),
            font_weight: "bold".to_owned(),
        };
        self.themes.insert("TEButton", style);

        let style = Style {
            name: "iconify.toolkit.TButton",
            background: "#1971c2".to_owned(),
            foreground: "#1e1e1e".to_owned(),
            font_size: (12.0 * GUI_RATIO) as u64,
            font_family: font_family.to_string(),
            font_weight: "bold".to_owned(),
        };
        self.themes.insert("TIButton", style);

        let style = Style {
            name: "toolkit.TNotebook",
            background: "#1e1e1e".to_owned(),
            ..Default::default()
        };
        self.themes.insert("TNotebook", style);

        let style = Style {
            name: "toolkit.TNotebook.Tab",
            background: "#ffffff".to_owned(),
            foreground: "#1e1e1e".to_owned(),
            font_size: (12.0 * GUI_RATIO) as u64,
            font_family: font_family.to_string(),
            ..Default::default()
        };
        self.themes.insert("notebook", style);

        self.themes.iter().for_each(|(_, style)| style.update());
    }

    fn build_window(&mut self) {
        let window = self.window.as_ref().unwrap();
        window.title("Afrim Wish");
        window.resizable(false, false);
        window.background("#1e1e1e");
        window.geometry(
            (480.0 * GUI_RATIO) as u64,
            (250.0 * GUI_RATIO) as u64,
            -1,
            -1,
        );

        // Header
        let frame = rstk::make_frame(window);
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
            let window = window.clone();
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
        rstk::make_frame(window)
            .pack()
            .fill(PackFill::X)
            .padx((30.0 * GUI_RATIO) as u64)
            .layout();
        let frame = rstk::make_frame(window);
        frame.style(&self.themes["TFrame"]);
        frame
            .pack()
            .fill(PackFill::X)
            .pady((10.0 * GUI_RATIO) as u64)
            .layout();

        // Body
        let notebook = rstk::make_notebook(window);
        notebook.style(&self.themes["TNotebook"]);

        // Page builder
        macro_rules! make_page {
            ( $tabname: expr, $($fieldname: expr => $fieldvalue: expr => $see_more: stmt)*) => {
                let frame = rstk::make_frame(window);
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
        let info = self.config.info.to_owned().unwrap_or_default();
        let core = self.config.core.to_owned().unwrap_or_default();
        make_page!(
            "Details",
            "IME:" => &info.input_method => {
                let window = window.clone();
                let config_name = info.name.to_owned();
                let config_maintainors = info.maintainors.join(", ");
                let config_homepage = info.homepage.clone();

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
            "Auto Commit:" => &core.auto_commit.to_string() => || ()
            "Buffer Size:" => &core.buffer_size.to_string() => || ()
        );

        // Help page
        make_page!(
            "Help",
            "Keyboard shortcuts:" => "open" => {
                let window = window.clone();

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
                let window = window.clone();

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

    pub fn build(&mut self, window: rstk::TkTopLevel) {
        self.window = Some(window);
        self.build_theme();
        self.build_window();
    }
}
