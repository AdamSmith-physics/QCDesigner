use gpui::*;
use gpui_component::{
    tab::TabBar,
    v_flex
};
use crate::models::Circuit;

use crate::utils::constants;

pub struct GateSelectorView {
    circuit: Entity<Circuit>,
}

impl GateSelectorView {
    pub fn new(circuit: Entity<Circuit>, _: &mut Window, cx: &mut Context<Self>) -> Self {
        cx.observe(&circuit, |_,_entity, cx| {
            cx.notify()
        }).detach();
        
        Self {
            circuit: circuit,
        }
    }
}

impl Render for GateSelectorView {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {

        let circuit = self.circuit.read(cx);
        let last_selected = circuit.last_clicked;
        let selected_string = match last_selected {
            Some(coord) => {
                let (row, col) = (coord.row, coord.column);
                format!("({row}, {col})")
            },
            None => {
                format!("None selected!")
            }
        };
        
        v_flex()
            .size_full()
            .child(
                TabBar::new("underline")
                    .w_full()
                    .selected_index(0)
                    .child(constants::gate_selector::PANEL_TITLE)
            )
            .child(
                div()
                    .p_2()
                    .child("Gate that was last selected")
                    .child(selected_string)
            )
            
        
    }
}
