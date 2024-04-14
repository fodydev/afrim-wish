use rstk::*;

pub fn init_rstk_ext() {
    // We create some useful tcl functions
    rstk::tell_wish("chan configure stdin -encoding utf-8");
}

#[derive(Clone, Debug, Default)]
pub struct Style {
    pub name: &'static str,
    pub background: String,
    pub foreground: String,
    pub font_size: u64,
    pub font_family: String,
    pub font_weight: String,
}

impl Style {
    pub fn update(&self) {
        rstk::tell_wish(&format!(
            "ttk::style layout {{{}}} [ttk::style layout {{{}}}];",
            self.name, self.name
        ));
        rstk::tell_wish(&format!(
            "ttk::style config {} -background {{{}}} -foreground {{{}}} -font {{{} {} {}}};",
            self.name,
            self.background,
            self.foreground,
            self.font_family,
            self.font_size,
            self.font_weight
        ));
    }
}

pub trait TkWidgetExt {
    fn style(&self, _style: &Style) {}
}

impl<T: TkWidget> TkWidgetExt for T {
    fn style(&self, style: &Style) {
        rstk::tell_wish(&format!(
            "{} configure -style {{{}}}",
            self.id(),
            style.name
        ));
    }
}

pub trait TkTopLevelExt {
    fn border(&self, _value: bool) {}
    fn topmost(&self, _value: bool) {}
    fn position(&self, _x: u64, _y: u64) {}
}

impl TkTopLevelExt for TkTopLevel {
    fn border(&self, value: bool) {
        rstk::tell_wish(&format!("wm overrideredirect {} {};", &self.id, !value));
    }

    fn topmost(&self, value: bool) {
        rstk::tell_wish(&format!("wm attributes {} -topmost {};", &self.id, value));
    }

    fn position(&self, x: u64, y: u64) {
        rstk::tell_wish(&format!("wm geometry {} +{}+{}", &self.id, x, y));
    }
}
