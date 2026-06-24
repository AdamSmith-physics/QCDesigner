use gpui::Global;
use gpui_component::ThemeMode;


pub struct AppSettings {
    pub theme: ThemeMode,
}

impl Global for AppSettings {}

#[allow(dead_code)]
impl AppSettings {
    pub fn default() -> Self {
        Self {
            theme: ThemeMode::Light,
        }
    }

    pub fn set_dark_theme(&mut self){
        self.theme = ThemeMode::Dark;
    }

    pub fn set_light_theme(&mut self) {
        self.theme = ThemeMode::Light;
    }

    pub fn toggle_theme(&mut self) {
        self.theme = match self.theme {
            ThemeMode::Light => ThemeMode::Dark,
            ThemeMode::Dark => ThemeMode::Light,
        };
    }
}