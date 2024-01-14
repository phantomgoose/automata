use crate::{CellState, SimulationState, BOARD_SIZE};

/// Gets the state of the cell at given coords while performing bounds checks on row/col
pub fn get_cell_with_bounds_check<Matrix: AsRef<[Row]>, Row: AsRef<[CellState]>>(
    state: &Matrix,
    row: Option<usize>,
    column: Option<usize>,
) -> Option<CellState> {
    let row = row?;
    let column = column?;
    state
        .as_ref()
        .get(row)
        .and_then(|row| row.as_ref().get(column))
        .copied()
}

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
    if row > 0 && column < BOARD_SIZE - 1 && predicate(state[row - 1][column + 1]) {
        count += 1;
    }

    // down
    if row < BOARD_SIZE - 1 && predicate(state[row + 1][column]) {
        count += 1;
    }

    // down left
    if row < BOARD_SIZE - 1 && column > 0 && predicate(state[row + 1][column - 1]) {
        count += 1;
    }

    // down right
    if row < BOARD_SIZE - 1 && column < BOARD_SIZE - 1 && predicate(state[row + 1][column + 1]) {
        count += 1;
    }

    // left
    if column > 0 && predicate(state[row][column - 1]) {
        count += 1;
    }

    // right
    if column < BOARD_SIZE - 1 && predicate(state[row][column + 1]) {
        count += 1;
    }

    count
}
