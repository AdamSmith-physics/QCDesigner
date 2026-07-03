use gpui::{*};
use gpui_component::{
    Icon, Sizable, TitleBar, Theme,
    h_flex, v_flex, 
    button::*, 
    tab::*,
    divider::Divider,
    label::Label,
    resizable::{
        h_resizable, resizable_panel, v_resizable
    }
};
use crate::views::{ CircuitSettingsView, GateSelectorView, GateSettingsView, CircuitView };
use crate::utils::{ constants, dimensions };
use crate::models::{ AppSettings, Circuit };

// --- end of imports ---

pub struct RootView {
    // Views
    circuit_view: Entity<CircuitView>,
    gate_selector_view: Entity<GateSelectorView>,
    circuit_settings_view: Entity<CircuitSettingsView>,
    gate_settings_view: Entity<GateSettingsView>,

    // Private fields
    active_tab_ix: usize,
}

impl RootView {
    
    pub fn new(window: &mut Window, cx: &mut App) -> Self {

        let circuit = cx.new(|_| Circuit::default());
        
        let circuit_view = cx.new(|cx| {
            CircuitView::new(circuit.clone(), window, cx)
        });
        let gate_selector_view = cx.new(|cx| {
            GateSelectorView::new(circuit.clone(), window, cx)
        });
        let circuit_settings_view = cx.new(|cx| {
            CircuitSettingsView::new(circuit.clone(), window, cx)
        });
        let gate_settings_view = cx.new(|cx| {
            GateSettingsView::new(circuit.clone(), window, cx)
        });

        Self { 
            circuit_view: circuit_view,
            gate_selector_view: gate_selector_view,
            circuit_settings_view: circuit_settings_view,
            gate_settings_view: gate_settings_view,
            active_tab_ix: 0
        }
    }
    
    fn set_active_tab(&mut self, ix: usize, _: &mut Window, cx: &mut Context<Self>) {
        self.active_tab_ix = ix;
        cx.notify();
    }

    fn render_titlebar() -> impl IntoElement {
        TitleBar::new()
            .child(
                Label::new(constants::titlebar::TITLE_PLACEHOLDER)
                .px(dimensions::PADDING)
            )
            .child(
                h_flex()
                    .id("titlebar-actions")
                    .px(dimensions::PADDING)
                    .size_full()
                    .justify_end()
                    .child(
                        Button::new("theme toggle")
                            .small()
                            .label("toggle")
                            .with_variant(ButtonVariant::Ghost)
                            .on_click(|_, _, cx| {
                                // This stops the double click to maximise on titlebar behind button
                                cx.stop_propagation();
                                let app_settings = cx.global_mut::<AppSettings>();
                                app_settings.toggle_theme();
                                Theme::change(app_settings.theme, None, cx);
                            })
                    )
                    .child(
                        Button::new("export") 
                            .small()
                            .icon(Icon::default().path("assets/share.svg"))
                            .label(constants::titlebar::EXPORT_LABEL)
                            .with_variant(ButtonVariant::Ghost)
                            .on_click(|_, _, cx| {
                                // This stops the double click to maximise on titlebar behind button
                                cx.stop_propagation();
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
                                        .size(dimensions::root_view::RIGHT_PANEL_SIZE)
                                        .size_range(dimensions::root_view::RIGHT_PANEL_MIN..dimensions::root_view::RIGHT_PANEL_MAX)
                                        .child(gate_selector_view)
                                    )
                            ))
                            .child(resizable_panel()
                                .size(dimensions::root_view::BOTTOM_PANEL_SIZE)
                                .size_range(dimensions::root_view::BOTTOM_PANEL_MIN..dimensions::root_view::BOTTOM_PANEL_MAX)
                                .child(
                                    v_flex()
                                        .size_full()
                                        .items_center()
                                        .child(
                                            TabBar::new("settings selector")
                                                .w_full()
                                                .menu(false)
                                                .selected_index(self.active_tab_ix)
                                                .on_click(cx.listener(|this, ix: &usize, window, cx| {
                                                    this.set_active_tab(*ix, window, cx);
                                                }))
                                                .child(constants::root_view::GATE_SETTINGS)
                                                .child(constants::root_view::CIRCUIT_SETTINGS)
                                        )
                                        .child(right_panel)
                                    )
                            )
                    )
            )
    }
}