use gpui::*;
use gpui_component::{
    button::{Button, Toggle, ToggleGroup, ToggleVariants},
    input::{InputEvent, InputState, NumberInput, NumberInputEvent, StepAction},
    Sizable,
    v_flex, h_flex,
};
use crate::{models::Circuit, utils::defaults::render_settings};
use crate::utils::dimensions;

// --- end of imports ---

/// View for editing global circuit properties (row count, etc.).
pub struct CircuitSettingsView {
    // Models
    circuit: Entity<Circuit>,

    // Other Entities
    num_qubits_input: Entity<InputState>,
    gate_size_input: Entity<InputState>,
    line_thickness_input: Entity<InputState>,
    corner_radius_input: Entity<InputState>,
    row_gap_input: Entity<InputState>,
    column_gap_input: Entity<InputState>,

    // Private fields
    checked: Vec<bool>,

     // Subscriptions
     _subscriptions: Vec<Subscription>,
}

impl CircuitSettingsView {
    pub fn new(circuit: Entity<Circuit>, window: &mut Window, cx: &mut Context<Self>) -> Self {
        cx.observe(&circuit, |_,_entity, cx| {
            cx.notify()
        }).detach();

        let render_settings = circuit.read(cx).render_settings;

        let num_qubits_input = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder("Integer")
                .default_value(circuit.read(cx).rows.to_string())
        });

        let gate_size_input = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder("Float")
                .default_value(render_settings.gate_size.to_string())
        });

        let line_thickness_input = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder("Integer")
                .default_value(render_settings.line_thickness.to_string())
        });

        let corner_radius_input = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder("Integer")
                .default_value(render_settings.corner_radius.to_string())
        });

        let row_gap_input = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder("Integer")
                .default_value(render_settings.row_gap.to_string())
        });

        let column_gap_input = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder("Integer")
                .default_value(render_settings.column_gap.to_string())
        });

        let _subscriptions = vec![
            cx.subscribe_in(&num_qubits_input, window, Self::on_input_event),
            cx.subscribe_in(&num_qubits_input, window, Self::on_number_input_event),
            cx.subscribe_in(&gate_size_input, window, Self::on_input_event),
            cx.subscribe_in(&gate_size_input, window, Self::on_number_input_event),
            cx.subscribe_in(&line_thickness_input, window, Self::on_input_event),
            cx.subscribe_in(&line_thickness_input, window, Self::on_number_input_event),
            cx.subscribe_in(&corner_radius_input, window, Self::on_input_event),
            cx.subscribe_in(&corner_radius_input, window, Self::on_number_input_event),
            cx.subscribe_in(&row_gap_input, window, Self::on_input_event),
            cx.subscribe_in(&row_gap_input, window, Self::on_number_input_event),
            cx.subscribe_in(&column_gap_input, window, Self::on_input_event),
            cx.subscribe_in(&column_gap_input, window, Self::on_number_input_event),
        ];
        
        Self {
            circuit: circuit,
            num_qubits_input: num_qubits_input,
            gate_size_input: gate_size_input,
            line_thickness_input: line_thickness_input,
            corner_radius_input: corner_radius_input,
            row_gap_input: row_gap_input,
            column_gap_input: column_gap_input,
            checked: vec![false; 10],
            _subscriptions: _subscriptions,
        }
    }

     // --- Input handlers ---

    /// Reacts to text changes and Enter key in the row-count input.
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

    /// Handles increment/decrement from the NumberInput step buttons.
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

// --- Focusable ---

impl Focusable for CircuitSettingsView {
    fn focus_handle(&self, cx: &gpui::App) -> gpui::FocusHandle {
        self.num_qubits_input.focus_handle(cx)
    }
}

// --- Render ---

impl Render for CircuitSettingsView {
    fn render(&mut self, _: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        
        v_flex()
            .p_2()
            .gap_2()
            .size_full()
            .items_center()
            // .child("This is the Circuit Settings View!")
            // .child(
            //     Button::new("ok")
            //         .label("Let's Go!")
            //         .on_click(|_, _, _| println!("Clicked!")),
            // )
            // .child(
            //     ToggleGroup::new("toggle-button-group-segmented-outline")
            //         .small()
            //         .outline()
            //         .children((0..10).map(|row| {
            //             Toggle::new(row).label(format!("{}", row)).checked(self.checked[row])
            //         }))
            // )
            .child(
                h_flex()
                    .w_full()
                    .justify_start()
                    .child("Number of qubits")
            )
            .child(
                h_flex()
                    .justify_center()
                    .min_w(dimensions::NUMBER_INPUT_WIDTH)
                    .child(NumberInput::new(&self.num_qubits_input).small())
            )
            .child(
                h_flex()
                    .w_full()
                    .justify_start()
                    .child("Gate Size")
            )
            .child(
                h_flex()
                    .justify_center()
                    .min_w(dimensions::NUMBER_INPUT_WIDTH)
                    .child(NumberInput::new(&self.gate_size_input).small())
            )
            .child(
                h_flex()
                    .w_full()
                    .justify_start()
                    .child("Line thickness")
            )
            .child(
                h_flex()
                    .justify_center()
                    .min_w(dimensions::NUMBER_INPUT_WIDTH)
                    .child(NumberInput::new(&self.line_thickness_input).small())
            )
            .child(
                h_flex()
                    .w_full()
                    .justify_start()
                    .child("Corner radius")
            )
            .child(
                h_flex()
                    .justify_center()
                    .min_w(dimensions::NUMBER_INPUT_WIDTH)
                    .child(NumberInput::new(&self.corner_radius_input).small())
            )
            .child(
                h_flex()
                    .w_full()
                    .justify_start()
                    .child("Row gap")
            )
            .child(
                h_flex()
                    .justify_center()
                    .min_w(dimensions::NUMBER_INPUT_WIDTH)
                    .child(NumberInput::new(&self.row_gap_input).small())
            )
            .child(
                h_flex()
                    .w_full()
                    .justify_start()
                    .child("Column gap")
            )
            .child(
                h_flex()
                    .justify_center()
                    .min_w(dimensions::NUMBER_INPUT_WIDTH)
                    .child(NumberInput::new(&self.column_gap_input).small())
            )
    }
}
