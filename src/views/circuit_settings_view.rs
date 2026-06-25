use gpui::*;
use gpui_component::{
    button::{Button, Toggle, ToggleGroup, ToggleVariants},
    Sizable,
    v_flex,
};

pub struct CircuitSettingsView {
    checked: Vec<bool>,
}

impl CircuitSettingsView {
    pub fn new(_: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|_cx| {
            Self {
                checked: vec![false; 10],
            }
        })
    }
}

impl Render for CircuitSettingsView {
    fn render(&mut self, _: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .p_2()
            .gap_2()
            .size_full()
            .items_center()
            .child("This is the Circuit Settings View!")
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
}
