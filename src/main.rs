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
actions!(app, [ExitFullscreen]);

fn quit(_: &Quit, cx: &mut App) {
    cx.quit();
}

fn exit_fullscreen(_: &ExitFullscreen, cx: &mut App) {
    if let Some(window) = cx.active_window() {
        // Defer the update until the current keystroke event is finished
        cx.defer(move |cx| {
            let result = window.update(cx, |_root: AnyView, window: &mut Window, _cx: &mut App| {
                if window.is_fullscreen(){
                    window.toggle_fullscreen();
                };
            });
            
            if let Err(e) = result {
                eprintln!("Failed to update window: {:?}", e);
            }
        });
    }
}

fn main() {
    Application::new().with_assets(CompositeAssets).run(move |cx| {
        // This must be called before using any GPUI Component features.
        gpui_component::init(cx);
        Theme::change(ThemeMode::Light, None, cx);

        cx.activate(true);
        cx.on_action(quit);
        cx.on_action(exit_fullscreen);

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

        cx.bind_keys([
            KeyBinding::new("Escape", ExitFullscreen, None),
        ]);

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
