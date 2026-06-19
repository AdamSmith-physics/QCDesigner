use gpui::*;
use gpui_component::{
    ActiveTheme, Icon, Root, Sizable, Theme, ThemeMode, TitleBar, StyledExt, button::*, divider::Divider, h_flex, input::*, label::Label, resizable::{h_resizable, resizable_panel, v_resizable}, tab::*, v_flex
};

mod assets;
use assets::CompositeAssets;

pub struct RootView {
    active_tab_ix: usize,
    checked: Vec<bool>,
}

impl RootView {
    
    fn new(_: &mut Window, _cx: &mut Context<Self>) -> Self {
        Self { 
            active_tab_ix: 0,
            checked: vec![false; 10]
        }
    }
    
    fn set_active_tab(&mut self, ix: usize, _: &mut Window, cx: &mut Context<Self>) {
        self.active_tab_ix = ix;
        cx.notify();
    }

    fn render_right_panel(&self, cx: &Context<Self>) -> impl IntoElement {
        v_flex()
            .gap_2()
            .size_full()
            .items_center()
            .child(
                TabBar::new("underline")
                    .w_full()
                    .menu(false)
                    .selected_index(self.active_tab_ix)
                    .on_click(cx.listener(|this, ix: &usize, window, cx| {
                        this.set_active_tab(*ix, window, cx);
                    }))
                    .child("Gate Settings")
                    .child("Circuit Setting")
            )
            .child(format!("Selected tab: {}", self.active_tab_ix))
            .child(
                Button::new("ok")
                    .label("Let's Go!")
                    .on_click(|_, _, _| println!("Clicked!")),
            )
            .child(
                ToggleGroup::new("toggle-button-group-segmented-outline")
                    .small()
                    .outline()
                    .children((0..10).map(|row| {
                        Toggle::new(row).label(format!("{}", row)).checked(self.checked[row])
                    }))
            )
    }

    fn render_titlebar() -> impl IntoElement {
        TitleBar::new()
            .child(Label::new("Title").px_2())
            .child(
                h_flex()
                    .id("titlebar-actions")
                    .px_2()
                    .size_full()
                    .justify_end()
                    .child(
                        Button::new("ok") 
                            .small()
                            .icon(Icon::default().path("assets/share.svg"))
                            .label("Export")
                            .with_variant(ButtonVariant::Ghost)
                            .on_click(|_, _, cx| {
                                cx.stop_propagation();
                                println!("Clicked!");
                            })
                    )
            )
    }

    fn render_left_content() -> impl IntoElement {
        v_flex()
            .gap_2()
            .size_full()
            .items_center()
            .justify_center()
            .child("Hello, World!")
            .child(
                Button::new("ok")
                    .label("Let's Go!")
                    .on_click(|_, _, _| println!("Clicked!")),
            )
    }

    fn render_gate_selector() -> impl IntoElement {
        v_flex()
            .size_full()
            .child(
                TabBar::new("underline")
                    .w_full()
                    .selected_index(0)
                    .child("Gate Selector")
            )
            .child("Content")
    }

}

impl Render for RootView {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let titlebar = Self::render_titlebar();
        let left_content = Self::render_left_content();
        let gate_selector = Self::render_gate_selector();
        let right_panel = self.render_right_panel(cx);

        div()
            .size_full()
            .child(
                v_flex()
                    .size_full()
                    .child(titlebar)
                    .child(Divider::horizontal())
                    .child(
                        h_resizable("main")
                            .child(resizable_panel().child(
                                v_resizable("vertical resize")
                                    .child(resizable_panel().child(left_content))
                                    .child(resizable_panel()
                                        .size(px(280.))
                                        .size_range(px(200.)..px(400.))
                                        .child(gate_selector)
                                    )
                            ))
                            .child(resizable_panel()
                                .size(px(280.))
                                .size_range(px(200.)..px(400.))
                                .child(right_panel)
                            )
                    )
            )
    }
}

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