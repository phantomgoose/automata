use crate::{CellState, SimulationState, COLUMNS, ROWS};

/// Computes the count of cells of interest surrounding the current cell in the matrix
pub fn count_cells<F>(state: &SimulationState, row: usize, column: usize, predicate: F) -> u8
where
    F: Fn(CellState) -> bool,
{
    let mut count = 0;

    // up
    if row > 0 && predicate(state[row - 1][column]) {
        count += 1;
    }

    // up left
    if row > 0 && column > 0 && predicate(state[row - 1][column - 1]) {
        count += 1;
    }

    // up right
    if row > 0 && column < COLUMNS - 1 && predicate(state[row - 1][column + 1]) {
        count += 1;
    }

    // down
    if row < ROWS - 1 && predicate(state[row + 1][column]) {
        count += 1;
    }

    // down left
    if row < ROWS - 1 && column > 0 && predicate(state[row + 1][column - 1]) {
        count += 1;
    }

    // down right
    if row < ROWS - 1 && column < COLUMNS - 1 && predicate(state[row + 1][column + 1]) {
        count += 1;
    }

    // left
    if column > 0 && predicate(state[row][column - 1]) {
        count += 1;
    }

    // right
    if column < COLUMNS - 1 && predicate(state[row][column + 1]) {
        count += 1;
    }

    count
}
