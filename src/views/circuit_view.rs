use gpui::*;
use crate::models::Circuit;
use crate::utils::{Coordinate, GateType, dimensions};
use crate::components::{ ScrollCenter, MeasuredElement, add_gate_button, gate_button };

// --- end of imports ---


/// Renders the circuit grid with gate buttons and background wires.
pub struct CircuitView {
    // Models
    circuit: Entity<Circuit>,

    // Private fields
    scroll_handle: ScrollHandle,
    /// Taffy-resolved content width from the previous frame; fed back into
    /// ScrollCenter so content doesn't collapse when the viewport is narrower.
    content_width: Option<Pixels>,
}

impl CircuitView {
    pub fn new(circuit: Entity<Circuit>, _: &mut Window, cx: &mut Context<Self>) -> Self {

        cx.observe(&circuit, |_,_entity, cx| {
            cx.notify()
        }).detach();
        
        Self {
            circuit: circuit,
            scroll_handle: ScrollHandle::new(),
            content_width: None,
        }
    }

    pub fn deselect(&mut self, cx: &mut Context<Self>) {
        cx.listener(move |this: &mut CircuitView, _: &MouseUpEvent, _, cx| {
            this.circuit.update(cx, move |circ, cx: &mut Context<Circuit>| {
                circ.deselect_gate();
            })      
        }); 
    }
}

impl Render for CircuitView {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {

        // --- Content-width measurement callback ---
        let weak = cx.weak_entity();
        let on_width_cb = move |width: Pixels, window: &mut Window, cx: &mut App| {
            window.defer(cx, move |_window, cx| {
                if let Some(entity) = weak.upgrade() {
                    entity.update(cx, |this: &mut CircuitView, cx| {
                        if this.content_width != Some(width) {
                            this.content_width = Some(width);
                            cx.notify();
                        }
                    });
                }
            });
        };
       
        
        let circuit = self.circuit.read(cx);
        let render_settings = circuit.render_settings;
        let num_rows = circuit.rows;
        let num_cols = circuit.cols;
        
        let mut col_divs: Vec<AnyElement> = Vec::new();
        for col in 0..num_cols {
            let mut col_elems: Vec<AnyElement> = Vec::new();
            let mut row = 0usize;
            while row < num_rows {

                let coord = Coordinate {row: row, column: col};

                let check_gate = circuit.get_gate_at_coordinate(&coord);

                match check_gate {
                    Some(gate) => {
                        let gate_for_listener = gate.clone();
                        col_elems.push(gate_button(
                            render_settings,
                            gate.clone(),
                            circuit.is_selected(gate),
                            cx.listener(move |this: &mut CircuitView, _, b, cx| {
                                let gate = gate_for_listener.clone();
                                this.circuit.update(cx, move |circ, cx: &mut Context<Circuit>| {
                                    // circ.remove_gate(&coord);
                                    circ.select_gate(gate);
                                    println!("button clicked!");
                                    cx.notify();
                                });
                            }),
                        ));
                        match gate.gate_type {
                            GateType::SingleQubit => row += 1,
                            _ => row += 1,
                        }
                    }
                    None => {
                        col_elems.push(add_gate_button(
                            render_settings,
                            cx.listener(move |this: &mut CircuitView, _, _, cx| {
                                this.circuit.update(cx, move |circ, cx: &mut Context<Circuit>| {
                                    circ.add_gate(coord);
                                    println!("button clicked!");
                                    cx.notify();
                                });
                            }),
                        ));
                        row += 1;
                    }
                }
            }
        
            col_divs.push(
                div()
                    .flex()
                    .flex_col()
                    .items_center()
                    .gap(px(render_settings.row_gap))
                    .children(col_elems).into_any_element(),
            );
        }


        // --- Background wire canvas ---
        let wire_canvas = canvas(
            |_bounds, _window, _cx| {},
            move |bounds, _state, window, _cx| {
                for row in 0..num_rows {
                    let y = bounds.origin.y
                        + px(row as f32 * (render_settings.gate_size + render_settings.row_gap) + render_settings.gate_size / 2.0);
                    window.paint_quad(fill(
                        Bounds {
                            origin: point(bounds.origin.x, y - px(render_settings.line_thickness / 2.0)),
                            size:   size(bounds.size.width, px(render_settings.line_thickness)),
                        },
                        black(),
                    ));
                }
            },
        )
        .absolute().top(px(0.0)).bottom(px(0.0)).left(px(0.0)).right(px(0.0));
        

        let content = div()
            .flex().flex_row().items_start()
            .px(px(render_settings.column_gap)) // Controls how much the wires stick out!
            .child(wire_canvas)
            .gap(px(render_settings.column_gap))
            .children(col_divs);
    
        let measured_content = MeasuredElement::new(content).on_width(on_width_cb);
    
        // --- Root element ---
        div().flex_1().min_w(px(0.0)).flex().flex_col()
            .child( 
                ScrollCenter::new(self.scroll_handle.clone(), measured_content)
                    .p(dimensions::PADDING)
                    .min_content_width(self.content_width),
            )
    }
}
