use crate::util::get_cell_count;
use crate::{CellState, SimulationState};

// brian's brain
pub fn get_brain_next_cell_state(state: &SimulationState, r: usize, c: usize) -> CellState {
    if state[r][c] == CellState::Alive {
        return CellState::Dying;
    }

    if state[r][c] == CellState::Dying {
        return CellState::Dead;
    }

    // get count of surrounding cells of target type
    let target_count = get_cell_count(state, r, c, |target| target == CellState::Alive);
    if target_count == 2 {
        CellState::Alive
    } else {
        CellState::Dead
    }
}
