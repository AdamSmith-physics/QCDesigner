use gpui::{App, AppContext, Entity, SharedString};
use crate::components::LatexLabel;
use crate::utils::{GateId, GateType, Coordinate, SvgStore};
use crate::utils::constants::gate_button as constants;

#[derive(Clone,PartialEq,Eq)]
pub struct Gate {
    id: GateId,
    pub gate_type: GateType,
    slice: usize,
    qubits: Vec<usize>,
    pub label: Option<SharedString>,
    // Persistent entity holding the compiled LaTeX -> SVG result for this
    // gate's label. Created once here and updated in place via `set_label`,
    // so the (expensive) Typst compilation only happens when the label text
    // actually changes -- never on every render.
    latex_label: Entity<LatexLabel>,
    // future fields
}

impl Gate {
    pub fn new(gate_type: GateType, coordinate: Coordinate, cx: &mut App) -> Self {
        let label: SharedString = "U".into();
        let svg_store = cx.global::<SvgStore>().clone();
        let latex_label = cx.new(|_cx| {
            LatexLabel::new(label.clone(), constants::LATEX_FONT_SIZE, constants::BUTTON_FG, svg_store)
        });

        Self {
            id: GateId::next(),
            gate_type: gate_type,
            slice: coordinate.column,
            qubits: vec![coordinate.row],
            label: Some(label),
            latex_label,
        }
    }

    pub fn id(&self) -> GateId {
        self.id
    }

    pub fn coordinate(&self) -> Coordinate {
        Coordinate { 
            row: *self.qubits.iter().min().expect("Gate should have at least one qubit!"), 
            column: self.slice 
        }
    }

    /// Entity backing the rendered LaTeX label for this gate. Clone this
    /// handle (cheap, refcounted) into element trees that need to display it
    /// -- e.g. `gate_button` adds it as a `.child(...)`.
    pub fn latex_label(&self) -> &Entity<LatexLabel> {
        &self.latex_label
    }

    /// Update this gate's displayed label, recompiling the LaTeX -> SVG only
    /// if the text actually changed (see `LatexLabel::set_latex`).
    pub fn set_label(&mut self, label: impl Into<SharedString>, cx: &mut App) {
        let label: SharedString = label.into();
        self.latex_label.update(cx, |latex_label, cx| {
            latex_label.set_latex(label.clone(), cx);
        });
        self.label = Some(label);
    }
}
