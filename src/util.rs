use crate::{CellState, SimulationState, COLUMNS, ROWS};

/// Fetches the number of cells of interest surrounding the current cell in the matrix
pub fn get_cell_count<F>(state: &SimulationState, r: usize, c: usize, predicate: F) -> u8
where
    F: Fn(CellState) -> bool,
{
    let mut count = 0;

    // up
    if r > 0 && predicate(state[r - 1][c]) {
        count += 1;
    }

    // up left
    if r > 0 && c > 0 && predicate(state[r - 1][c - 1]) {
        count += 1;
    }

    // up right
    if r > 0 && c < COLUMNS - 1 && predicate(state[r - 1][c + 1]) {
        count += 1;
    }

    // down
    if r < ROWS - 1 && predicate(state[r + 1][c]) {
        count += 1;
    }

    // down left
    if r < ROWS - 1 && c > 0 && predicate(state[r + 1][c - 1]) {
        count += 1;
    }

    // down right
    if r < ROWS - 1 && c < COLUMNS - 1 && predicate(state[r + 1][c + 1]) {
        count += 1;
    }

    // left
    if c > 0 && predicate(state[r][c - 1]) {
        count += 1;
    }

    // right
    if c < COLUMNS - 1 && predicate(state[r][c + 1]) {
        count += 1;
    }

    count
}
