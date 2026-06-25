use gpui::*;
use gpui_component::{
    button::{Button, Toggle, ToggleGroup, ToggleVariants},
    Sizable,
    v_flex,
};
use crate::models::Circuit;

pub struct GateSettingsView {  
    circuit: Entity<Circuit>, 
    
    checked: Vec<bool>,
}

impl GateSettingsView {
    pub fn new(circuit: Entity<Circuit>, _: &mut Window, cx: &mut Context<Self>) -> Self {
        cx.observe(&circuit, |_,_entity, cx| {
            cx.notify()
        }).detach();
        
        Self {
            circuit: circuit,
            checked: vec![false; 10],
        }
    }
}

impl Render for GateSettingsView {
    fn render(&mut self, _: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .p_2()
            .gap_2()
            .size_full()
            .items_center()
            .child("This is the Gate Settings View!")
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
