use gpui::*;
use gpui_component::{
    Theme,
    button::Button,
    v_flex
};
use crate::models::{Coordinate, Circuit};
use crate::components::{ ScrollCenter, MeasuredElement, gate_button };

pub struct CircuitView {
    circuit: Entity<Circuit>,

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
        //     v_flex()
        //         .gap_2()
        //         .size_full()
        //         .items_center()
        //         .justify_center()
        //         .child("This is the circuit view!")
        //         .child(
        //             Button::new("ok")
        //                 .label("Let's Go!")
        //                 .on_click(|_, _, _| println!("Clicked!")),
        //         )
        // }
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
    
        // // ── Snapshot state ────────────────────────────────────────────────────
        // // Read everything needed for rendering in one borrow, then drop it so
        // // cx is free for cx.listener calls in the grid loop below.
        // let (button_size, row_gap, col_gap, line_thickness, num_rows, num_cols, has_cnot) = {
        //     let circuit = self.circuit.read(cx);
        //     let s     = &state.settings;
        //     let c     = &state.circuit;
        //     // Pre-compute the full CNOT occupancy matrix so we never need to
        //     // re-borrow state inside the loop.
        //     let has_cnot: Vec<Vec<bool>> = (0..c.cols())
        //         .map(|col| (0..c.rows()).map(|row| c.has_cnot_at(col, row)).collect())
        //         .collect();
        //     (s.button_size, s.row_gap, s.col_gap, s.line_thickness, c.rows(), c.cols(), has_cnot)
        // };
        // let bg = cx.theme().background;
    
        // // ── Grid ──────────────────────────────────────────────────────────────
        // let mut col_divs: Vec<AnyElement> = Vec::new();
        // for col in 0..num_cols {
        //     let mut col_elems: Vec<AnyElement> = Vec::new();
        //     let mut row = 0usize;
        //     while row < num_rows {
        //         if has_cnot[col][row] {
        //             col_elems.push(cnot_element(
        //                 bg, button_size, row_gap, line_thickness,
        //                 cx.listener(move |this: &mut CircuitView, _, _, cx| {
        //                     this.state.update(cx, |s, cx: &mut Context<AppState>| {
        //                         s.circuit.remove_cnot(col, row);
        //                         cx.notify();
        //                     });
        //                 }),
        //             ));
        //             row += 2;
        //         } else {
        //             col_elems.push(gate_button(
        //                 button_size,
        //                 cx.listener(move |this: &mut CircuitView, _, _, cx| {
        //                     this.state.update(cx, |s, cx: &mut Context<AppState>| {
        //                         s.circuit.place_cnot(col, row);
        //                         s.last_clicked = Some((col, row));
        //                         cx.notify();
        //                     });
        //                 }),
        //             ));
        //             row += 1;
        //         }
        //     }
        //     col_divs.push(
        //         div().flex().flex_col().gap(px(row_gap)).children(col_elems).into_any_element(),
        //     );
        // }
    
        // let grid = div()
        //     .flex().flex_row().items_start()
        //     .gap(px(col_gap))
        //     .children(col_divs);
    
        // // ── Background wire canvas ────────────────────────────────────────────
        // let wire_canvas = canvas(
        //     |_bounds, _window, _cx| {},
        //     move |bounds, _state, window, _cx| {
        //         for row in 0..num_rows {
        //             let y = bounds.origin.y
        //                 + px(row as f32 * (button_size + row_gap) + button_size / 2.0);
        //             window.paint_quad(fill(
        //                 Bounds {
        //                     origin: point(bounds.origin.x, y - px(line_thickness / 2.0)),
        //                     size:   size(bounds.size.width, px(line_thickness)),
        //                 },
        //                 black(),
        //             ));
        //         }
        //     },
        // )
        // .absolute().top(px(0.0)).bottom(px(0.0)).left(px(0.0)).right(px(0.0));

        let circuit = self.circuit.read(cx);
        
        let mut col_divs: Vec<AnyElement> = Vec::new();
        for col in 0..circuit.cols {
            let mut col_elems: Vec<AnyElement> = Vec::new();
            let mut row = 0usize;
            while row < circuit.rows {
                
                col_elems.push(gate_button(
                    30.0,
                    cx.listener(move |this: &mut CircuitView, _, _, cx| {
                        this.circuit.update(cx, move |circ, cx: &mut Context<Circuit>| {
                            let coordinate = Coordinate{ row: row, column: col };
                            circ.add_gate(coordinate);
                            println!("button clicked!");
                            cx.notify();
                        });
                    }),
                ));
                row += 1;
            }
        
            col_divs.push(
                div().flex().flex_col().gap(px(8.0)).children(col_elems).into_any_element(),
            );
        }

        let content = div()
            .flex().flex_row().items_start()
            .gap(px(8.0))
            .children(col_divs);
    
        let measured_content = MeasuredElement::new(content).on_width(on_width_cb);
    
        // ── Root element ──────────────────────────────────────────────────────
        div().flex_1().min_w(px(0.0)).flex().flex_col()
            .child( 
                ScrollCenter::new(self.scroll_handle.clone(), measured_content)
                    .p(px(8.0))
                    .min_content_width(self.content_width),
            )
    }
}
