use gpui::*;
use gpui_component::{
    button::{Button, Toggle, ToggleGroup, ToggleVariants},
    input::{InputEvent, InputState, NumberInput, NumberInputEvent, StepAction},
    Sizable,
    v_flex, h_flex,
};
use crate::models::Circuit;

pub struct CircuitSettingsView {
    circuit: Entity<Circuit>,

    number_input1: Entity<InputState>,
    
    checked: Vec<bool>,

    _subscriptions: Vec<Subscription>,
}

impl CircuitSettingsView {
    pub fn new(circuit: Entity<Circuit>, window: &mut Window, cx: &mut Context<Self>) -> Self {
        cx.observe(&circuit, |_,_entity, cx| {
            cx.notify()
        }).detach();

        let number_input1 = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder("Normal Integer")
                .default_value(circuit.read(cx).rows.to_string())
        });

        let _subscriptions = vec![
            cx.subscribe_in(&number_input1, window, Self::on_input_event),
            cx.subscribe_in(&number_input1, window, Self::on_number_input_event),
        ];
        
        Self {
            circuit: circuit,
            number_input1: number_input1,
            checked: vec![false; 10],
            _subscriptions: _subscriptions,
        }
    }

    fn on_input_event(
        &mut self,
        state: &Entity<InputState>,
        event: &InputEvent,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match event {
            InputEvent::Change => {
                let text = state.read(cx).value();
                if let Ok(value) = text.parse::<i64>() {
                    self.circuit.update(cx, |circuit, cx| {
                        circuit.set_rows(value);
                        cx.notify();
                    });
                }
                println!("Change: {}", text);
            }
            InputEvent::PressEnter { secondary } => {
                println!("PressEnter secondary: {}", secondary)
            }
            InputEvent::Focus => println!("Focus"),
            InputEvent::Blur => println!("Blur"),
        }
    }

    fn on_number_input_event(
        &mut self,
        this: &Entity<InputState>,
        event: &NumberInputEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match event {
            NumberInputEvent::Step(step_action) => {
                self.circuit.update(cx, |circuit, cx| {
                    match step_action {
                        StepAction::Decrement => circuit.decrease_rows(),
                        StepAction::Increment => circuit.increase_rows(),
                    }
                    cx.notify();
                });
                this.update(cx, |input, cx| {
                    input.set_value(self.circuit.read(cx).rows.to_string(), window, cx);
                });
            }
        }
    }  
}

impl Focusable for CircuitSettingsView {
    fn focus_handle(&self, cx: &gpui::App) -> gpui::FocusHandle {
        self.number_input1.focus_handle(cx)
    }
}

impl Render for CircuitSettingsView {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        
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
            .child(
                h_flex()
                    .justify_center()
                    .min_w(px(150.0))
                    .child(NumberInput::new(&self.number_input1))
            )
    }
}
