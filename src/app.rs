use gpui::*;

// ── Actions ─────────────────────────────────────────────────────────────

actions!(gpui_menu, [SaveFile, Quit]);
actions!(app, [ExitFullscreen]);

pub fn init(cx: &mut App) {
    // Register action handlers
    cx.on_action(quit);
    cx.on_action(exit_fullscreen);

    // ── Menu bar setup ──────────────────────────────────────────────────────
    cx.set_menus(vec![
        Menu {
            name: "gpui-menu".into(),
            items: vec![
                MenuItem::action("Quit", Quit),
            ],
        },
        Menu {
            name: "File".into(),
            items: vec![
                MenuItem::action("Save", SaveFile),
            ],
        },
    ]);

    // ── Key bindings ────────────────────────────────────────────────────────
    cx.bind_keys([
        KeyBinding::new("Escape", ExitFullscreen, None),
    ]);
}

// ── Handlers ────────────────────────────────────────────────────────────────

fn quit(_: &Quit, cx: &mut App) {
    cx.quit();
}

fn exit_fullscreen(_: &ExitFullscreen, cx: &mut App) {
    if let Some(window) = cx.active_window() {
        cx.defer(move |cx| {
            let result = window.update(cx, |_root: AnyView, window: &mut Window, _cx: &mut App| {
                if window.is_fullscreen() {
                    window.toggle_fullscreen();
                }
            });

            if let Err(e) = result {
                eprintln!("Failed to update window: {:?}", e);
            }
        });
    }
}
