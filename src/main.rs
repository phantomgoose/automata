use macroquad::prelude::*;

#[derive(Clone, Copy, PartialOrd, PartialEq, Debug)]
enum CellState {
    Alive,
    // Dying,
    Dead,
}

impl CellState {
    fn color(&self) -> Color {
        match self {
            CellState::Alive => GREEN,
            // CellState::Dying => ORANGE,
            CellState::Dead => BLACK,
        }
    }
}

const ROWS: usize = 128;
const COLUMNS: usize = 128;

type SimState = [[CellState; COLUMNS]; ROWS];

/// Fetches the number of cells of interest surrounding the current cell in the matrix
fn get_cell_count<F>(state: SimState, r: usize, c: usize, predicate: F) -> u8
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

// brian's brain
// TODO: convert to trait?
// fn get_next_cell_state(state: SimState, r: usize, c: usize) -> CellState {
//     if state[r][c] == CellState::Alive {
//         return CellState::Dying;
//     }
//
//     if state[r][c] == CellState::Dying {
//         return CellState::Dead;
//     }
//
//     // get count of surrounding cells of target type
//     let target_count = get_cell_count(state, r, c, |target| target == CellState::Alive);
//     if target_count == 2 {
//         CellState::Alive
//     } else {
//         CellState::Dead
//     }
// }

// Conway's game of life
fn get_next_cell_state(state: SimState, r: usize, c: usize) -> CellState {
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

#[macroquad::main("Life")]
async fn main() {
    request_new_screen_size(1024., 1024.);
    next_frame().await;

    // create a 2d array of size w * h
    let mut state: SimState = [[CellState::Dead; COLUMNS]; ROWS];
    let mut temp: SimState = [[CellState::Dead; COLUMNS]; ROWS];

    assert_eq!(
        std::mem::size_of_val(&state),
        std::mem::size_of_val(&temp),
        "State and temp matrices are not of the same size"
    );

    // init state
    // for r in 0..state.len() {
    //     for c in 0..state.len() {
    //         if rand::gen_range(0, 5) == 0 {
    //             state[r][c] = CellState::Alive;
    //         }
    //     }
    // }

    let desired_cell_size: f32 = screen_width() / COLUMNS as f32;

    // main simulation loop
    loop {
        // clear_background(WHITE);

        // break
        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        // spawn alive cells on mouse click
        if is_mouse_button_down(MouseButton::Left) {
            let (x, y) = mouse_position();
            let column = (x / desired_cell_size) as usize;
            let row = (y / desired_cell_size) as usize;

            // bounds check
            if (row > 0 && row < ROWS - 1) && (column > 0 && column < COLUMNS - 1) {
                state[row][column] = CellState::Alive;

                state[row - 1][column] = CellState::Alive;
                state[row + 1][column] = CellState::Alive;

                state[row][column - 1] = CellState::Alive;
                state[row][column + 1] = CellState::Alive;
            }
        }

        // update cell state
        for r in 0..state.len() {
            for c in 0..state[r].len() {
                temp[r][c] = get_next_cell_state(state, r, c);
            }
        }

        // render cell state
        for r in 0..temp.len() {
            for c in 0..temp[r].len() {
                let cell = temp[r][c];

                // update state
                state[r][c] = cell;

                // bg size - 1 px to create a nice juicy border
                let curr_cell_size = desired_cell_size - 1.;

                draw_rectangle(
                    c as f32 * desired_cell_size,
                    r as f32 * desired_cell_size,
                    desired_cell_size,
                    desired_cell_size,
                    BLACK,
                );

                draw_rectangle(
                    c as f32 * desired_cell_size + 1.,
                    r as f32 * desired_cell_size + 1.,
                    curr_cell_size,
                    curr_cell_size,
                    cell.color(),
                );
            }
        }

        next_frame().await
    }
}
