// --- Crate modules ---
mod views;
mod assets;
mod models;
mod app;
mod utils;
mod components;

// --- Imports ---
use gpui::*;
use gpui_component::{
    Root, Theme, ThemeMode, TitleBar,
};

use views::RootView;
use assets::CompositeAssets;
use models::AppSettings;
use utils::SvgStore;

// --- End of imports ---


fn main() {
    // Created once here so we can register it both as GPUI's registered
    // AssetSource (via CompositeAssets) and as a global (so any `&mut App` /
    // `&mut Context<T>` can fetch a clone with `cx.global::<SvgStore>()`).
    let svg_store = SvgStore::new();

    Application::new()
        .with_assets(CompositeAssets::new(svg_store.clone()))
        .run(move |cx| {
            // This must be called before using any GPUI Component features.
            gpui_component::init(cx);
            Theme::change(ThemeMode::Light, None, cx);

            cx.set_global(svg_store);

            cx.activate(true);
            app::init(cx);

            AppSettings::set_global(cx, AppSettings::default());

            cx.on_window_closed(|cx| cx.quit()).detach();  // close app when closing the window.

            cx.spawn(async move |cx| {
                let window_options = WindowOptions {
                    titlebar: Some(TitleBar::title_bar_options()),
                    ..Default::default()
                };

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
