use crate::util::get_cell_count;
use crate::{CellState, SimulationState};

// Conway's game of life
pub fn get_conway_next_cell_state(state: &SimulationState, r: usize, c: usize) -> CellState {
    // get count of surrounding cells of target type
    let target_count = get_cell_count(state, r, c, |target| target == CellState::Alive);

    if target_count < 2 || target_count > 3 {
        return CellState::Dead;
    }

    if target_count == 3 {
        return CellState::Alive;
    }

    state[r][c]
}
