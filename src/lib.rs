mod config;
mod window;

use afrim::frontend::Frontend;
use rstk::*;
use window::{rstk_ext::init_rstk_ext, toolkit::ToolKit, tooltip::ToolTip};

pub use config::Config;

#[derive(Clone)]
pub struct Wish {
    window: rstk::TkTopLevel,
    tooltip: ToolTip,
    toolkit: ToolKit,
}

impl Wish {
    pub fn init(config: config::Config) -> Self {
        let wish = if cfg!(debug_assertions) {
            rstk::trace_with("wish").unwrap()
        } else {
            rstk::start_wish().unwrap()
        };

        init_rstk_ext();

        let tooltip = ToolTip::new(config.theme.to_owned().unwrap_or_default());
        let toolkit = ToolKit::new(config.to_owned());

        Wish {
            window: wish,
            tooltip,
            toolkit,
        }
    }

    pub fn raise_error(&self, message: &str, detail: &str) {
        rstk::message_box()
            .parent(&self.window)
            .icon(IconImage::Error)
            .title("Unexpected Error")
            .message(message)
            .detail(detail)
            .show();
        rstk::end_wish();
    }

    pub fn build(&mut self) {
        self.tooltip.build(rstk::make_toplevel(&self.window));
        self.toolkit.build(self.window.to_owned());
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
        self.tooltip.update_screen(screen);
    }

    fn update_position(&mut self, position: (f64, f64)) {
        self.tooltip.update_position(position);
    }

    fn set_input(&mut self, text: &str) {
        self.tooltip.set_input_text(text);
    }

    fn set_page_size(&mut self, size: usize) {
        self.tooltip.set_page_size(size);
    }

    fn add_predicate(&mut self, code: &str, remaining_code: &str, text: &str) {
        self.tooltip.add_predicate(code, remaining_code, text);
    }

    fn clear_predicates(&mut self) {
        self.tooltip.clear();
    }

    fn previous_predicate(&mut self) {
        self.tooltip.select_previous_predicate();
    }

    fn next_predicate(&mut self) {
        self.tooltip.select_next_predicate();
    }

    fn get_selected_predicate(&self) -> Option<&(String, String, String)> {
        self.tooltip.get_selected_predicate()
    }

    fn display(&self) {
        self.tooltip.update();
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
        afrim_wish.destroy();
    }
}
