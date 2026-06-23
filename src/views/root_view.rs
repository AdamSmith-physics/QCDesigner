use gpui::{prelude::FluentBuilder, *};
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
use crate::views::{CircuitSettingsView, GateSelectorView, GateSettingsView, circuit_settings_view, gate_selector_view};

use super::CircuitView;

actions!(root_view, [ExitFullscreen]);

pub struct RootView {
    circuit_view: Entity<CircuitView>,
    gate_selector_view: Entity<GateSelectorView>,
    circuit_settings_view: Entity<CircuitSettingsView>,
    gate_settings_view: Entity<GateSettingsView>,
    
    active_tab_ix: usize,
    checked: Vec<bool>,
}

impl RootView {
    
    pub fn new(window: &mut Window, cx: &mut App) -> Self {
        cx.bind_keys([
                KeyBinding::new("Escape", ExitFullscreen, None),
            ]);

        let circuit_view = CircuitView::new(window, cx);
        let gate_selector_view = GateSelectorView::new(window, cx);
        let circuit_settings_view = CircuitSettingsView::new(window, cx);
        let gate_settings_view = GateSettingsView::new(window, cx);

        Self { 
            circuit_view: circuit_view,
            gate_selector_view: gate_selector_view,
            circuit_settings_view: circuit_settings_view,
            gate_settings_view: gate_settings_view,
            active_tab_ix: 0,
            checked: vec![false; 10]
        }
    }
    
    fn set_active_tab(&mut self, ix: usize, _: &mut Window, cx: &mut Context<Self>) {
        self.active_tab_ix = ix;
        cx.notify();
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

}

impl Render for RootView {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let titlebar = Self::render_titlebar();

        let circuit_view = self.circuit_view.clone();
        let gate_selector_view = self.gate_selector_view.clone();
        let circuit_settings_view = self.circuit_settings_view.clone();
        let gate_settings_view = self.gate_settings_view.clone();

        let right_panel = match self.active_tab_ix {
            0 => div().child(gate_settings_view),
            _ => div().child(circuit_settings_view),
        };
        
        

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
                                    .child(resizable_panel()
                                        .child(circuit_view)
                                    )
                                    .child(resizable_panel()
                                        .size(px(280.))
                                        .size_range(px(200.)..px(400.))
                                        .child(gate_selector_view)
                                    )
                            ))
                            .child(resizable_panel()
                                .size(px(280.))
                                .size_range(px(200.)..px(400.))
                                .child(
                                    v_flex()
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
                                        .child(right_panel)
                                    )
                            )
                    )
            )
    }
}