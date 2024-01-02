use macroquad::prelude::*;

use crate::brain::get_brain_next_cell_state;
use crate::conway::get_conway_next_cell_state;

mod brain;
mod conway;
mod util;

const ROWS: usize = 128;
const COLUMNS: usize = 128;

#[derive(Clone, Copy, PartialOrd, PartialEq, Debug)]
enum CellState {
    Alive,
    Dying,
    Dead,
}

enum SimulationMode {
    ConwayGameOfLife,
    BrianBrain,
}

impl SimulationMode {
    fn cell_state_fn(&self) -> CellStateGenerator {
        match self {
            SimulationMode::BrianBrain => get_brain_next_cell_state,
            SimulationMode::ConwayGameOfLife => get_conway_next_cell_state,
        }
    }
}

impl CellState {
    fn color(&self) -> Color {
        match self {
            CellState::Alive => BLUE,
            CellState::Dying => LIGHTGRAY,
            CellState::Dead => BLACK,
        }
    }
}

type SimulationState = [[CellState; COLUMNS]; ROWS];

type CellStateGenerator = fn(&SimulationState, usize, usize) -> CellState;

/// Given the starting simulation state, update the buffer using the supplied update func
fn get_next_state(
    state: &SimulationState,
    buffer: &mut SimulationState,
    update_func: CellStateGenerator,
) {
    for r in 0..state.len() {
        for c in 0..state[r].len() {
            buffer[r][c] = update_func(&state, r, c);
        }
    }
}

fn get_clear_sim_state() -> SimulationState {
    [[CellState::Dead; COLUMNS]; ROWS]
}

fn generate_state_and_buffer() -> (SimulationState, SimulationState) {
    let state = get_clear_sim_state();
    let buffer = get_clear_sim_state();
    assert_eq!(
        std::mem::size_of_val(&state),
        std::mem::size_of_val(&buffer),
        "State and buffer matrices are not of the same size"
    );

    (state, buffer)
}

fn reset_sim_state(state: &mut SimulationState, buffer: &mut SimulationState) {
    (*state, *buffer) = generate_state_and_buffer()
}

fn randomize_sim_state(state: &mut SimulationState, buffer: &mut SimulationState) {
    reset_sim_state(state, buffer);
    for r in 0..state.len() {
        for c in 0..state.len() {
            if rand::gen_range(0, 5) == 0 {
                state[r][c] = CellState::Alive;
            }
        }
    }
}

fn select_sim_mode(current_mode: &mut SimulationMode, new_mode: SimulationMode) {
    *current_mode = new_mode;
}

#[macroquad::main("Life")]
async fn main() {
    request_new_screen_size(1024., 1024.);
    next_frame().await;

    // create a 2d array of size w * h
    let (mut state, mut buffer) = generate_state_and_buffer();

    let mut mode = SimulationMode::ConwayGameOfLife;

    let desired_cell_size: f32 = screen_width() / COLUMNS as f32;

    let instructions = vec![
        "Controls:",
        "R -> Clear",
        "A -> Randomize",
        "B -> Brian's Brain",
        "C -> Conway's Game of Life",
    ];

    // main simulation loop
    loop {
        // exit
        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        // spawn live cells on mouse click
        if is_mouse_button_down(MouseButton::Left) {
            let (x, y) = mouse_position();
            let column = (x / desired_cell_size) as usize;
            let row = (y / desired_cell_size) as usize;

            // bounds check
            if (row > 0 && row < ROWS - 1) && (column > 0 && column < COLUMNS - 1) {
                state[row][column] = CellState::Alive;

                state[row + 1][column] = CellState::Alive;
                state[row][column + 1] = CellState::Alive;
                state[row + 1][column + 1] = CellState::Alive;
            }
        }

        // reset the state
        if is_key_pressed(KeyCode::R) {
            reset_sim_state(&mut state, &mut buffer);
        }

        // randomize the state
        if is_key_pressed(KeyCode::A) {
            randomize_sim_state(&mut state, &mut buffer);
        }

        // select conway's game of life
        if is_key_pressed(KeyCode::C) {
            reset_sim_state(&mut state, &mut buffer);
            randomize_sim_state(&mut state, &mut buffer);
            select_sim_mode(&mut mode, SimulationMode::ConwayGameOfLife);
        }

        // select brian's brain
        if is_key_pressed(KeyCode::B) {
            reset_sim_state(&mut state, &mut buffer);
            randomize_sim_state(&mut state, &mut buffer);
            select_sim_mode(&mut mode, SimulationMode::BrianBrain);
        }

        // update cell state
        get_next_state(&state, &mut buffer, mode.cell_state_fn());

        // render cell state
        for r in 0..buffer.len() {
            for c in 0..buffer[r].len() {
                let cell = buffer[r][c];

                // update state
                state[r][c] = cell;

                // size - 1 px to create a nice juicy border
                let cell_size = desired_cell_size - 1.;

                draw_rectangle(
                    c as f32 * desired_cell_size,
                    r as f32 * desired_cell_size,
                    cell_size,
                    cell_size,
                    cell.color(),
                );
            }
        }

        let mut text_y = 25.;
        let font_size = 24.;
        let font_color = WHITE;
        for instruction in &instructions {
            draw_text(instruction, 25., text_y, font_size, font_color);
            text_y += font_size + 5.;
        }

        next_frame().await
    }
}
