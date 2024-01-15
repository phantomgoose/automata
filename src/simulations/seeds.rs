use crate::util::count_cells;
use crate::{CellState, SimulationState};

// Seeds variation of the Game of Life
pub fn get_seeds_next_cell_state(state: &SimulationState, row: usize, column: usize) -> CellState {
    let live_cell_count = count_cells(state, row, column, |cell| cell == CellState::Leaf);

    match state[row][column] {
        CellState::Leaf => CellState::Empty,
        CellState::Empty => {
            if live_cell_count == 2 {
                CellState::Leaf
            } else {
                CellState::Empty
            }
        }
        _ => CellState::Empty,
    }
}
