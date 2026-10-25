// --- Crate modules ---
mod views;
mod assets;
mod models;
mod app;
mod utils;
mod components;

// --- Imports ---
use gpui_kit::*;
use gpui_kit::component::{
    Root, Theme, ThemeMode, TitleBar,
};

use views::RootView;
use assets::CompositeAssets;
use models::AppSettings;

// --- End of imports ---


fn main() {
    gpui_kit::application()
        .with_assets(CompositeAssets)
        .run(move |cx| {
            // This must be called before using any GPUI Kit / Component features.
            gpui_kit::init(cx);
            Theme::change(ThemeMode::Light, None, cx);

            cx.activate(true);
            app::init(cx);

            AppSettings::set_global(cx, AppSettings::default());

            cx.on_window_closed(|cx, _window_id| cx.quit()).detach();  // close app when closing the window.

            cx.spawn(async move |cx| {
                let window_options = TitleBar::window_options();

                let window = cx
                    .open_window(window_options, |_window, _cx| {
                        let view = _cx.new(|cx| RootView::new(_window, cx));
                        // This first level on the window, should be a Root.
                        _cx.new(|cx| Root::new(view, _window, cx))
                    })?;

                window.update(cx, |_, window, _| {
                    window.activate_window();
                })?;

                Ok::<_, anyhow::Error>(())
            })
            .detach();
        });
}
