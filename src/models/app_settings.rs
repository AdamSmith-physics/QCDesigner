use gpui::Global;
use gpui_component::ThemeMode;

/// Global application settings — theme mode.
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

     // --- Theme ---

    pub fn set_dark_theme(&mut self) {
        self.theme = ThemeMode::Dark;
     }

    pub fn set_light_theme(&mut self) {
        self.theme = ThemeMode::Light;
     }

    /// Switch between light and dark mode.
    pub fn toggle_theme(&mut self) {
        self.theme = match self.theme {
            ThemeMode::Light => ThemeMode::Dark,
            ThemeMode::Dark => ThemeMode::Light,
        };
    }
}