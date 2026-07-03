use gpui::*;
use crate::models::{Coordinate, Circuit};
use crate::components::{ ScrollCenter, MeasuredElement, add_gate_button, gate_button };

// --- end of imports ---


// Temporary constants
const BUTTON_SIZE: f32 = 30.0;
const LINE_THICKNESS: f32 = 1.0;
const ROW_GAP: f32 = 8.0;



/// --- CircuitView ---
/// View for showing the circuit. Wraps the circuit in ScrollCenter
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
}

impl Render for CircuitView {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {

        // ── Content-width measurement callback ────────────────────────────────
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
        let num_rows = circuit.rows;
        let num_cols = circuit.cols;
        
        let mut col_divs: Vec<AnyElement> = Vec::new();
        for col in 0..num_cols {
            let mut col_elems: Vec<AnyElement> = Vec::new();
            let mut row = 0usize;
            while row < num_rows {

                let coord = Coordinate {row: row, column: col};

                match circuit.is_selected(&coord) {

                    // Add button for adding gates
                    false => {
                        col_elems.push(add_gate_button(
                            BUTTON_SIZE,
                            cx.listener(move |this: &mut CircuitView, _, _, cx| {
                                this.circuit.update(cx, move |circ, cx: &mut Context<Circuit>| {
                                    circ.add_gate(coord);
                                    println!("button clicked!");
                                    cx.notify();
                                });
                            }),
                        ));
                    },

                    // Add button with number selected.
                    true => {
                        col_elems.push(gate_button(
                            BUTTON_SIZE,
                            circuit.get_gate_number(&coord),
                            cx.listener(move |this: &mut CircuitView, _, _, cx| {
                                this.circuit.update(cx, move |circ, cx: &mut Context<Circuit>| {
                                    circ.remove_gate(&coord);
                                    println!("button clicked!");
                                    cx.notify();
                                });
                            }),
                        ));
                    }
                };
                row += 1;
            }
        
            col_divs.push(
                div().flex().flex_col().gap(px(ROW_GAP)).children(col_elems).into_any_element(),
            );
        }


        // ── Background wire canvas ---
        let wire_canvas = canvas(
            |_bounds, _window, _cx| {},
            move |bounds, _state, window, _cx| {
                for row in 0..num_rows {
                    let y = bounds.origin.y
                        + px(row as f32 * (BUTTON_SIZE + ROW_GAP) + BUTTON_SIZE / 2.0);
                    window.paint_quad(fill(
                        Bounds {
                            origin: point(bounds.origin.x, y - px(LINE_THICKNESS / 2.0)),
                            size:   size(bounds.size.width, px(LINE_THICKNESS)),
                        },
                        black(),
                    ));
                }
            },
        )
        .absolute().top(px(0.0)).bottom(px(0.0)).left(px(0.0)).right(px(0.0));
        

        let content = div()
            .flex().flex_row().items_start()
            .px(px(ROW_GAP)) // Controls how much the wires stick out!
            .child(wire_canvas)
            .gap(px(ROW_GAP))
            .children(col_divs);
    
        let measured_content = MeasuredElement::new(content).on_width(on_width_cb);
    
        // ── Root element ---
        div().flex_1().min_w(px(0.0)).flex().flex_col()
            .child( 
                ScrollCenter::new(self.scroll_handle.clone(), measured_content)
                    .p(px(8.0))
                    .min_content_width(self.content_width),
            )
    }
}
