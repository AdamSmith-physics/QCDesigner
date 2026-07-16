use gpui::*;
use gpui_component::{
    Sizable, 
    button::{Button, Toggle, ToggleGroup, ToggleVariants}, 
    input::{InputState, InputEvent, Input}, 
    divider::Divider,
    switch::Switch,
    v_flex, h_flex,
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
            cx.observe_in(&circuit, window, |this, _circuit, window, cx| {
                let label_input = this.label_input.clone();
                this.update_strings(&label_input, window, cx);
                cx.notify()
            }),
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
                         selected_gate.set_label(text, cx);
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
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {

        let circuit = self.circuit.read(cx);
        let selected_gate_id = circuit.selected_gate;
        let render_latex = circuit.selected_gate().map(|gate| gate.render_latex()).unwrap_or(false);

        // Div to be shown if no gate is selected
        let none_div = v_flex()
             .p_2()
             .gap_2()
             .size_full()
             .items_center()
             .child("Please select a gate!");

        
        // Settings shown when gate is selected
        let some_div = v_flex()
            .p_2()
            .gap_2()
            .size_full()
            .items_center()
            .child(
                h_flex()
                    .w_full()
                    .justify_start()
                    .child("Gate Label")
            )
            .child(
                h_flex()
                    .justify_center()
                    .w_full()
                    .child(Input::new(&self.label_input).cleanable(true))
            )
            .child(
                h_flex()
                    .w_full()
                    .justify_between()
                    .items_center()
                    .child("Render as LaTeX")
                    .child(
                        Switch::new("render-latex-switch")
                            .small()
                            .checked(render_latex)
                            .on_click(cx.listener(|this, checked: &bool, _window, cx| {
                                let checked = *checked;
                                this.circuit.update(cx, |circuit, cx| {
                                    if let Some(gate) = circuit.selected_gate_mut() {
                                        gate.set_render_latex(checked);
                                    }
                                    cx.notify();
                                });
                            }))
                    )
            )
            .child(Divider::horizontal().w_full().pt_2());


        match selected_gate_id {
            Some(_) => {
                some_div
            },
            None => {
                none_div
            }
        }

        
     }
}
