use gpui::*;
use gpui_component::{
    Icon, Sizable, TitleBar, 
    h_flex, v_flex, 
    button::*, 
    tab::*,
    divider::Divider,
    label::Label,
    resizable::{
        h_resizable, resizable_panel, v_resizable
    }
};

actions!(root_view, [ExitFullscreen]);

pub struct RootView {
    active_tab_ix: usize,
    checked: Vec<bool>,
}

impl RootView {
    
    pub fn new(_: &mut Window, cx: &mut Context<Self>) -> Self {
        cx.bind_keys([
                KeyBinding::new("Escape", ExitFullscreen, None),
            ]);

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
            .on_action(|&ExitFullscreen, window, _cx| {
                    println!("div version triggered!");
                    window.toggle_fullscreen();
                })
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