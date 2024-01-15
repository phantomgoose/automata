use crate::util::count_cells;
use crate::{CellState, SimulationState};

// The HighLife variation of the Game of Life
pub fn get_highlife_next_cell_state(
    state: &SimulationState,
    row: usize,
    column: usize,
) -> CellState {
    let live_cell_count = count_cells(state, row, column, |cell| cell == CellState::Leaf);

    // underpopulated or overpopulated
    if live_cell_count < 2 || live_cell_count > 3 {
        return CellState::Empty;
    }

    // just right 👌
    if live_cell_count == 3 || live_cell_count == 6 {
        return CellState::Leaf;
    }

    // unchanged otherwise
    state[row][column]
}
