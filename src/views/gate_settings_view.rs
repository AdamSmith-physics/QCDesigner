use gpui::*;
use gpui_component::{
    Sizable, button::{Button, Toggle, ToggleGroup, ToggleVariants}, input::{InputState, InputEvent, Input}, v_flex, h_flex,
};
use crate::models::Circuit;
use crate::utils::dimensions;

#[allow(dead_code)]
/// View for configuring properties of a selected gate.
pub struct GateSettingsView {  
    // Models
    circuit: Entity<Circuit>, 

    // Inpuyt Entities
    label_input: Entity<InputState>,
    
    // Private fields
    checked: Vec<bool>,

    // Subscriptions
    _subsciptions: Vec<Subscription>,
}

impl GateSettingsView {
    pub fn new(circuit: Entity<Circuit>, window: &mut Window, cx: &mut Context<Self>) -> Self {
        cx.observe(&circuit, |_,_entity, cx| {
            cx.notify()
        }).detach();

        let gate_label = match circuit.read(cx).selected_gate() {
            Some(gate) => gate.label.clone(),
            None => None,
        };
        
        let label_input = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder("Gate Label")
                .default_value(gate_label.unwrap_or_default())
        });

        let _subscriptions = vec![
            cx.subscribe_in(&label_input, window, Self::on_input_event),
        ];
        
        Self {
            circuit: circuit,
            label_input: label_input,
            checked: vec![false; 10],
            _subsciptions: _subscriptions,
         }
     }


     fn update_strings(&mut self, state: &Entity<InputState>, window: &mut Window, cx: &mut Context<Self>){
        if state == &self.label_input {
            let label = self.circuit.read(cx)
                .selected_gate()
                .and_then(|gate| gate.label.clone())
                .unwrap_or_default();

            state.update(cx, |input, cx| {
                input.set_value(label, window, cx);
            });
        }
     }

     
     /// Reacts to text changes and Enter key in the row-count input.
     fn on_input_event(
         &mut self,
         state: &Entity<InputState>,
         event: &InputEvent,
         window: &mut Window,
         cx: &mut Context<Self>,
     ) {
         match event {
             InputEvent::Change => {},
             InputEvent::PressEnter { secondary } => {
                 let text = state.read(cx).value();
                 if state == &self.label_input {
                     self.circuit.update(cx, |circuit, cx| {
                        // Gate settings will only be shown if gate is selected
                         let selected_gate = circuit.selected_gate_mut().unwrap(); 
                         selected_gate.label = Some(text);
                         cx.notify();
                     })
                 }
                 self.update_strings(state, window, cx);
             },
             InputEvent::Focus => {},
             InputEvent::Blur => {},
         }

     }
}

// --- Render ---

impl Render for GateSettingsView {
    fn render(&mut self, _: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        // TODO: Replace placeholder UI with actual gate property controls.
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
             .child(
                h_flex()
                    .justify_center()
                    .min_w(dimensions::NUMBER_INPUT_WIDTH)
                    .child(Input::new(&self.label_input).cleanable(true))
             )
     }
}
