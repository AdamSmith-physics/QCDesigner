mod views;
mod assets;
mod app;
mod utils;

use gpui::*;
use gpui_component::{
    Root, Theme, ThemeMode, TitleBar,
};

use views::RootView;
use assets::CompositeAssets;

fn main() {
    Application::new().with_assets(CompositeAssets).run(move |cx| {
        // This must be called before using any GPUI Component features.
        gpui_component::init(cx);
        Theme::change(ThemeMode::Light, None, cx);

        cx.activate(true);
        app::init(cx);

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
