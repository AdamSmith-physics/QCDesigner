mod views;
mod assets;

use gpui::*;
use gpui_component::{
    Root, Theme, ThemeMode, TitleBar,
};

use views::RootView;
use assets::CompositeAssets;

// Declare actions in the "gpui_menu" namespace
actions!(gpui_menu, [SaveFile, Quit]);

fn quit(_: &Quit, cx: &mut App) {
    cx.quit();
}

fn main() {
    Application::new().with_assets(CompositeAssets).run(move |cx| {
        // This must be called before using any GPUI Component features.
        gpui_component::init(cx);
        Theme::change(ThemeMode::Light, None, cx);

        cx.activate(true);
        cx.on_action(quit);

        // ── Menu bar setup ────────────────────────────────────────────────────
        // `set_menus` replaces the default GPUI menu bar with our own.
        // Each `Menu` becomes one top-level item in the macOS menu bar.
        cx.set_menus(vec![
            // The first menu is the application menu (shows the app name on macOS).
            Menu {
                name: "gpui-menu".into(),
                items: vec![
                    // Quit item wired to the built-in macOS Quit key equivalent (⌘Q)
                    // using OsAction is optional; here we keep things minimal.
                    MenuItem::action("Quit", Quit),
                ],
            },
            // "File" menu with a single "Save" entry.
            Menu {
                name: "File".into(),
                items: vec![
                    MenuItem::action("Save", SaveFile),
                ],
            },
        ]);

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