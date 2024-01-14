use crate::util::get_cell_with_bounds_check;
use crate::{CellState, SimulationState};

pub mod trainer;

type Features = [u8; 5];
type Output = CellState;

// needed for a stable representation of the cell state values for ML purposes
impl From<CellState> for u8 {
    fn from(value: CellState) -> Self {
        match value {
            CellState::Dead => 0,
            CellState::Alive => 1,
            CellState::Dying => 2,
        }
    }
}

fn cell_to_u8(state: &SimulationState, row: Option<usize>, column: Option<usize>) -> u8 {
    let maybe_state: Option<CellState> = get_cell_with_bounds_check(state, row, column);

    maybe_state.map(|cell_state| cell_state as u8).unwrap_or(0)
}

fn state_to_model_vec(state: &SimulationState, row: usize, column: usize) -> Features {
    let left = cell_to_u8(state, Some(row), column.checked_sub(1));
    let right = cell_to_u8(state, Some(row), column.checked_add(1));
    let up = cell_to_u8(state, row.checked_sub(1), Some(column));
    let down = cell_to_u8(state, row.checked_add(1), Some(column));

    let center = cell_to_u8(state, Some(row), Some(column));

    [left, right, up, down, center]
}

fn predict(features: Features) -> Output {
    // this is actually a fun pattern somehow
    let sum: u8 = features.iter().sum();
    if sum % 2 == 0 {
        CellState::Alive
    } else {
        CellState::Dead
    }
}

pub fn predict_tree_next_cell_state(
    state: &SimulationState,
    row: usize,
    column: usize,
) -> CellState {
    let encoded = state_to_model_vec(state, row, column);

    predict(encoded)
}
