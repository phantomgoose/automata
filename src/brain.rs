use crate::util::count_cells;
use crate::{CellState, SimulationState};

// Brian's Brain
pub fn get_brain_next_cell_state(state: &SimulationState, row: usize, column: usize) -> CellState {
    // live cells begin dying
    if state[row][column] == CellState::Alive {
        return CellState::Dying;
    }

    // dying cells die
    if state[row][column] == CellState::Dying {
        return CellState::Dead;
    }

    // if there are
    let target_count = count_cells(state, row, column, |target| target == CellState::Alive);
    if target_count == 2 {
        CellState::Alive
    } else {
        CellState::Dead
    }
}
