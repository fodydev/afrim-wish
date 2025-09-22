mod config;
mod window;

use afrim::frontend::{Command, Frontend};
use afrish::*;
use anyhow::{anyhow, Result};
use std::sync::{
    mpsc::{Receiver, Sender},
    OnceLock,
};
use std::thread;
use window::{toolkit::ToolKit, tooltip::ToolTip};

pub use config::Config;

pub struct Wish {
    window: &'static afrish::TkTopLevel,
    tooltip: ToolTip,
    toolkit: ToolKit,
    tx: Option<Sender<Command>>,
    rx: Option<Receiver<Command>>,
}

impl Wish {
    // Extract a version from a wish binary name.
    // Examples:
    //  - "wish" => Some(0.0)
    //  - "wish9.0" => Some(9.0)
    //  - "wish8.6" => Some(8.6)
    //  - "wishx" => None (ignored)
    pub(crate) fn wish_parse_version(name: &str) -> Option<f32> {
        if !name.starts_with("wish") {
            return None;
        }

        let tail = &name[4..];
        let ver_str: String = tail
            .chars()
            .take_while(|c| c.is_ascii_digit() || *c == '.')
            .collect();

        if tail.is_empty() {
            return Some(0.0);
        }

        if ver_str.is_empty() {
            return None;
        }

        Some(ver_str.parse::<f32>().unwrap_or(0.0))
    }

    pub(crate) fn wish_sort_and_dedupe_bins(mut bins: Vec<(f32, String)>) -> Vec<String> {
        bins.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));
        let mut seen = std::collections::HashSet::new();
        let mut candidates: Vec<String> = Vec::new();
        for (_, p) in bins {
            if seen.insert(p.clone()) {
                candidates.push(p);
            }
        }
        candidates
    }

    pub(crate) fn wish_fallback_candidates() -> Vec<String> {
        vec!["wish9.0".into(), "wish8.7".into(), "wish8.6".into(), "wish".into()]
    }

    fn init() -> &'static afrish::TkTopLevel {
        static WISH: OnceLock<afrish::TkTopLevel> = OnceLock::new();
        WISH.get_or_init(|| {
            let wish = if cfg!(debug_assertions) {
                // Prefer a user-provided Wish binary on macOS where /usr/bin/wish is often Tk 8.5.
                let env_bin = std::env::var("WISH_BIN")
                    .or_else(|_| std::env::var("AFRISH_WISH_BIN"))
                    .ok();

                if let Some(bin) = env_bin {
                    afrish::trace_with(&bin).unwrap_or_else(|e| {
                        panic!(
                            "Failed to start Wish at '{}'. Error: {:?}",
                            bin, e
                        )
                    })
                } else {
                    let mut bins: Vec<(f32, String)> = Vec::new();
                    if let Ok(path_var) = std::env::var("PATH") {
                        for dir in std::env::split_paths(&path_var) {
                            if let Ok(read_dir) = std::fs::read_dir(&dir) {
                                for entry in read_dir.flatten() {
                                    let name_os = entry.file_name();
                                    if let Some(name) = name_os.to_str() {
                                        if let Some(ver) = Self::wish_parse_version(name) {
                                            let full = entry.path().to_string_lossy().to_string();
                                            bins.push((ver, full));
                                        }
                                    }
                                }
                            }
                        }
                    }

                    let mut candidates = Self::wish_sort_and_dedupe_bins(bins);
                    if candidates.is_empty() {
                        // Fallback to common names if PATH scan found nothing.
                        candidates = Self::wish_fallback_candidates();
                    }

                    let mut last_err: Option<(String, afrish::TkError)> = None;
                    for c in candidates {
                        match afrish::trace_with(&c) {
                            Ok(w) => return w,
                            Err(e) => last_err = Some((c, e)),
                        }
                    }

                    match last_err {
                        Some((cmd, err)) => panic!(
                            "Failed to launch Wish (last tried '{}'). Set WISH_BIN to your Tk wish (e.g., /opt/homebrew/opt/tcl-tk/bin/wish or wish9.0). Error: {:?}",
                            cmd, err
                        ),
                        None => panic!(
                            "Failed to launch Wish. Set WISH_BIN to your Tk wish (e.g., /opt/homebrew/opt/tcl-tk/bin/wish or wish9.0)."
                        ),
                    }
                }
            } else {
                afrish::start_wish().unwrap()
            };

            // The default behavior is to close the window.
            // But since this window, represent the main window,
            // we don't want an unexpected behavior.
            // It's better for us to manage the close.
            //
            // Note that, this close button is on the title bar.
            wish.on_close(Self::kill);

            wish
        })
    }

    pub fn from_config(config: config::Config) -> Self {
        let wish = Self::init();
        let tooltip = ToolTip::new(config.theme.to_owned().unwrap_or_default());
        let toolkit = ToolKit::new(config.to_owned());

        Wish {
            window: wish,
            tooltip,
            toolkit,
            tx: None,
            rx: None,
        }
    }

    pub fn raise_error<T: std::fmt::Debug>(message: &str, detail: T) {
        afrish::message_box()
            .parent(Self::init())
            .icon(IconImage::Error)
            .title("Unexpected Error")
            .message(message)
            .detail(&format!("{detail:?}"))
            .show();
        Self::kill();
    }

    fn build(&mut self) {
        self.tooltip.build(afrish::make_toplevel(self.window));
        self.toolkit.build(self.window.to_owned());
    }

    /// End the process (wish and rust).
    ///
    /// Note that a `process::exit` is called internally.
    pub fn kill() {
        afrish::end_wish();
    }
}

impl Frontend for Wish {
    fn init(&mut self, tx: Sender<Command>, rx: Receiver<Command>) -> Result<()> {
        self.tx = Some(tx);
        self.rx = Some(rx);
        self.build();

        Ok(())
    }
    fn listen(&mut self) -> Result<()> {
        if self.tx.as_ref().and(self.rx.as_ref()).is_none() {
            return Err(anyhow!("you should config the channel first!"));
        }

        // We shouldn't forget to listen for GUI events.
        thread::spawn(afrish::mainloop);

        let tx = self.tx.as_ref().unwrap();

        loop {
            let command = self.rx.as_ref().unwrap().recv()?;
            match command {
                Command::ScreenSize(screen) => self.tooltip.update_screen(screen),
                Command::Position(position) => self.tooltip.update_position(position),
                Command::InputText(input) => self.tooltip.set_input_text(input),
                Command::PageSize(size) => self.tooltip.set_page_size(size),
                Command::State(state) => self.toolkit.set_idle_state(state),
                Command::Predicate(predicate) => self.tooltip.add_predicate(predicate),
                Command::Update => self.tooltip.update(),
                Command::Clear => self.tooltip.clear(),
                Command::SelectPreviousPredicate => self.tooltip.select_previous_predicate(),
                Command::SelectNextPredicate => self.tooltip.select_next_predicate(),
                Command::SelectedPredicate => {
                    if let Some(predicate) = self.tooltip.get_selected_predicate() {
                        tx.send(Command::Predicate(predicate.to_owned()))?;
                    } else {
                        tx.send(Command::NoPredicate)?;
                    }
                }
                Command::NOP => {
                    if let Some(state) = self.toolkit.new_idle_state() {
                        tx.send(Command::State(state))?;
                    } else {
                        tx.send(Command::NOP)?;
                    }
                }
                Command::End => {
                    tx.send(Command::End)?;
                    self.window.destroy();

                    return Ok(());
                }
                _ => (),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Config, Wish};
    use afrim::frontend::{Command, Frontend, Predicate};
    use std::path::Path;
    use std::sync::mpsc;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_wish_parse_version() {
        assert_eq!(Wish::wish_parse_version("wish"), Some(0.0));
        assert_eq!(Wish::wish_parse_version("wish9.0"), Some(9.0));
        assert_eq!(Wish::wish_parse_version("wish8.6"), Some(8.6));
        assert_eq!(Wish::wish_parse_version("wish10"), Some(10.0));
        assert_eq!(Wish::wish_parse_version("wishx"), None);
        assert_eq!(Wish::wish_parse_version("bash"), None);
    }

    #[test]
    fn test_wish_sort_and_dedupe_bins() {
        let bins = vec![
            (8.6, "/bin/wish8.6".to_string()),
            (9.0, "/opt/wish9.0".to_string()),
            (0.0, "/usr/bin/wish".to_string()),
            (9.0, "/opt/wish9.0".to_string()), // duplicate path
        ];
        let out = Wish::wish_sort_and_dedupe_bins(bins);
        assert_eq!(out, vec![
            "/opt/wish9.0".to_string(),
            "/bin/wish8.6".to_string(),
            "/usr/bin/wish".to_string(),
        ]);
    }

    #[test]
    fn test_wish_fallback_candidates() {
        let fall = Wish::wish_fallback_candidates();
        assert_eq!(fall, vec![
            "wish9.0".to_string(),
            "wish8.7".to_string(),
            "wish8.6".to_string(),
            "wish".to_string(),
        ]);
    }

    #[test]
    fn test_api() {
        let config = Config::from_file(Path::new("data/full_sample.toml")).unwrap();
        let mut afrim_wish = Wish::from_config(config);
        assert!(afrim_wish.listen().is_err());
        let (tx1, rx1) = mpsc::channel();
        let (tx2, rx2) = mpsc::channel();

        let afrim_wish_thread = thread::spawn(move || {
            afrim_wish.init(tx2, rx1).unwrap();
            afrim_wish.listen().unwrap();
        });

        tx1.send(Command::NOP).unwrap();
        assert_eq!(rx2.recv().unwrap(), Command::NOP);

        // Test without data.
        tx1.send(Command::ScreenSize((480, 320))).unwrap();
        tx1.send(Command::Clear).unwrap();
        tx1.send(Command::SelectNextPredicate).unwrap();
        tx1.send(Command::SelectPreviousPredicate).unwrap();
        tx1.send(Command::SelectedPredicate).unwrap();
        assert_eq!(rx2.recv().unwrap(), Command::NoPredicate);
        tx1.send(Command::Update).unwrap();

        // Test the adding of predicates.
        tx1.send(Command::PageSize(3)).unwrap();
        tx1.send(Command::InputText("Test started!".to_owned()))
            .unwrap();
        tx1.send(Command::Predicate(Predicate {
            code: "test".to_owned(),
            remaining_code: "123".to_owned(),
            texts: vec!["ok".to_owned()],
            can_commit: false,
        }))
        .unwrap();
        tx1.send(Command::Predicate(Predicate {
            code: "test1".to_owned(),
            remaining_code: "23".to_owned(),
            texts: vec!["ok".to_owned()],
            can_commit: false,
        }))
        .unwrap();
        tx1.send(Command::Predicate(Predicate {
            code: "test12".to_owned(),
            remaining_code: "3".to_owned(),
            texts: vec!["ok".to_owned()],
            can_commit: false,
        }))
        .unwrap();
        tx1.send(Command::Predicate(Predicate {
            code: "test123".to_owned(),
            remaining_code: "".to_owned(),
            texts: vec!["ok".to_owned()],
            can_commit: false,
        }))
        .unwrap();
        tx1.send(Command::Predicate(Predicate {
            code: "test1234".to_owned(),
            remaining_code: "".to_owned(),
            texts: vec!["".to_owned()],
            can_commit: false,
        }))
        .unwrap();
        tx1.send(Command::Update).unwrap();

        // Test the geometry.
        (0..100).for_each(|i| {
            if i % 10 != 0 {
                return;
            };
            let i = i as f64;
            tx1.send(Command::Position((i, i))).unwrap();
            thread::sleep(Duration::from_millis(100));
        });

        // Test the navigation.
        tx1.send(Command::SelectPreviousPredicate).unwrap();
        tx1.send(Command::SelectedPredicate).unwrap();
        assert_eq!(
            rx2.recv().unwrap(),
            Command::Predicate(Predicate {
                code: "test123".to_owned(),
                remaining_code: "".to_owned(),
                texts: vec!["ok".to_owned()],
                can_commit: false,
            })
        );
        tx1.send(Command::SelectNextPredicate).unwrap();
        tx1.send(Command::SelectedPredicate).unwrap();
        assert_eq!(
            rx2.recv().unwrap(),
            Command::Predicate(Predicate {
                code: "test".to_owned(),
                remaining_code: "123".to_owned(),
                texts: vec!["ok".to_owned()],
                can_commit: false,
            })
        );
        tx1.send(Command::Update).unwrap();

        // Test the idle state.
        tx1.send(Command::State(true)).unwrap();
        tx1.send(Command::State(false)).unwrap();

        // We end the communication.
        tx1.send(Command::End).unwrap();
        assert_eq!(rx2.recv().unwrap(), Command::End);
        assert!(rx2.recv().is_err());

        // We wait the afrim to end properly.
        afrim_wish_thread.join().unwrap();
    }
}