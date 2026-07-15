use std::fmt;

/// Position in the circuit grid.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coordinate {
    pub row: usize,
    pub column: usize
}

impl fmt::Debug for Coordinate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Coord({}, {})", self.row, self.column)
    }
}
