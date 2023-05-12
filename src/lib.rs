use clafrica::api;
use rstk::*;
use std::{thread, time::Duration};

pub struct Wish {
    font_size: u64,
    border: f64,
    screen: (u64, u64),
    label: rstk::TkLabel,
    window: rstk::TkTopLevel,
}

impl Wish {
    pub fn build(label_length: usize) -> Self {
        let wish = rstk::start_wish().unwrap();

        wish.title("Clafrica Wish");
        wish.background("black");
        wish.resizable(false, false);
        wish.withdraw();
        {
            let wish_obj = &wish.id;

            rstk::tell_wish(&format!(
                r#"
                wm overrideredirect {wish_obj} 1;
                wm attributes {wish_obj} -topmost 1;
                bind {wish_obj} <Motion> {{destroy .; exit;}};
            "#
            ));
        }
        wish.deiconify();

        let label = rstk::make_label(&wish);
        label.text(&"abcdefghijklmnopqrstuvwxyz"[..label_length]);
        label.background("black");
        label.foreground("white");
        label.grid().layout();

        thread::sleep(Duration::from_millis(1000));

        Wish {
            window: wish,
            label,
            screen: (0, 0),
            border: 0.0,
            font_size: 0,
        }
    }

    fn refresh(&self) {
        self.label.font(&rstk::TkFont {
            family: String::from("Charis-SIL"),
            size: self.font_size,
            weight: rstk::Weight::Bold,
            ..Default::default()
        });
    }
}

impl api::Frontend for Wish {
    fn update_screen(&mut self, screen: (u64, u64)) {
        self.border = f64::sqrt((screen.0 * screen.0 + screen.1 * screen.1) as f64) / 100.0;
        self.font_size = self.border as u64;
        self.screen = screen;
        self.refresh();
    }

    fn update_position(&mut self, position: (f64, f64)) {
        {
            let wish_obj = &self.window.id;
            let x = (position.0 + self.border) as u64;
            let y = (position.1 + self.border) as u64;
            rstk::tell_wish(&format!("wm geometry {wish_obj} +{x}+{y}"));
        }
    }

    fn update_text(&mut self, text: Vec<char>) {
        let text = text
            .into_iter()
            .map(|c| c.to_string())
            .collect::<Vec<_>>()
            .join("");

        self.label.text(&text);

        if text == "_exit_" {
            rstk::end_wish();
        }
    }
}
