use crate::models::predict_tree_next_cell_state;
use crate::{CellState, SimulationState};

// Grow in a tree-like pattern
pub fn get_tree_next_cell_state(state: &SimulationState, row: usize, column: usize) -> CellState {
    predict_tree_next_cell_state(state, row, column)
}
