mod views;
mod assets;

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

        cx.on_window_closed(|cx| cx.quit()).detach();  // close app when closing the window.

        cx.spawn(async move |cx| {
            cx.open_window(WindowOptions {
                titlebar: Some(TitleBar::title_bar_options()),
                ..Default::default()
            }, |window, cx| {
                let view = cx.new(|cx| RootView::new(window,cx));
                // This first level on the window, should be a Root.
                cx.new(|cx| Root::new(view, window, cx))
            })?;

            Ok::<_, anyhow::Error>(())
        })
        .detach();
    });
}