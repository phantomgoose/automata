use macroquad::prelude::*;
use macroquad::rand::ChooseRandom;

use crate::brain::get_brain_next_cell_state;
use crate::conway::get_conway_next_cell_state;

mod brain;
mod conway;
mod util;

const ROWS: usize = 256;
const COLUMNS: usize = 256;
const FONT_SIZE: f32 = 24.;
const TEXT_PADDING: f32 = 25.;
const FONT_COLOR: Color = WHITE;
const INSTRUCTIONS: [&str; 7] = [
    "Controls:",
    "R -> Clear",
    "A -> Randomize",
    "B -> Brian's Brain",
    "C -> Conway's Game of Life",
    "LMB -> Spawn Live Cells",
    "ESC -> Quit",
];

#[derive(Clone, Copy, PartialEq, Debug)]
enum CellState {
    Alive,
    Dying,
    Dead,
}

impl CellState {
    fn color(&self) -> Color {
        match self {
            CellState::Alive => {
                let live_colors = vec![GREEN, BLUE, LIME];
                *live_colors.choose().unwrap()
            }
            CellState::Dying => LIGHTGRAY,
            CellState::Dead => BLACK,
        }
    }
}

#[derive(Default)]
enum SimulationMode {
    #[default]
    ConwaysLife,
    BriansBrain,
}

impl SimulationMode {
    fn cell_state_fn(&self) -> CellStateGenerator {
        match self {
            SimulationMode::BriansBrain => get_brain_next_cell_state,
            SimulationMode::ConwaysLife => get_conway_next_cell_state,
        }
    }
}

type SimulationState = [[CellState; COLUMNS]; ROWS];

type CellStateGenerator = fn(&SimulationState, usize, usize) -> CellState;

/// Given the starting simulation state, update each cell in the buffer using the supplied update func
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

fn get_clean_state() -> (SimulationState, SimulationState) {
    let state = [[CellState::Dead; COLUMNS]; ROWS];
    let buffer = [[CellState::Dead; COLUMNS]; ROWS];
    assert_eq!(
        std::mem::size_of_val(&state),
        std::mem::size_of_val(&buffer),
        "State and buffer matrices are not of the same size"
    );

    (state, buffer)
}

fn reset_sim_state(state: &mut SimulationState, buffer: &mut SimulationState) {
    (*state, *buffer) = get_clean_state()
}

/// Randomly sets cells in the starting state to [CellState::Alive]
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

#[macroquad::main("Automata")]
async fn main() {
    // set window size
    request_new_screen_size(1024., 1024.);
    next_frame().await;

    // create initial state and a buffer to hold updated state between frames
    let (mut state, mut buffer) = get_clean_state();

    let mut simulation_mode = SimulationMode::ConwaysLife;

    let cell_width: f32 = screen_width() / COLUMNS as f32;

    // main simulation loop
    loop {
        // exit (if not wasm)
        if is_key_pressed(KeyCode::Escape) && !cfg!(target_arch = "wasm32") {
            break;
        }

        // spawn live cells on mouse click
        if is_mouse_button_down(MouseButton::Left) {
            let (x, y) = mouse_position();
            let row = (y / cell_width) as usize;
            let column = (x / cell_width) as usize;

            // bounds check
            if (row > 0 && row < ROWS - 1) && (column > 0 && column < COLUMNS - 1) {
                // spawn a square around the mouse pointer - works well for the supported sims
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
            simulation_mode = SimulationMode::ConwaysLife;
        }

        // select brian's brain
        if is_key_pressed(KeyCode::B) {
            reset_sim_state(&mut state, &mut buffer);
            randomize_sim_state(&mut state, &mut buffer);
            simulation_mode = SimulationMode::BriansBrain;
        }

        // write updated cell state for the next frame to buffer, based on the currently selected simulation mode
        get_next_state(&state, &mut buffer, simulation_mode.cell_state_fn());

        // keep track of how many cells are alive
        let mut live_cell_count = 0;

        // render the cell state and store buffer in the state
        for r in 0..buffer.len() {
            for c in 0..buffer[r].len() {
                let cell = buffer[r][c];

                if cell == CellState::Alive {
                    live_cell_count += 1;
                }

                // update state
                state[r][c] = cell;

                // size - 1 px to create a nice juicy border
                let cell_size = cell_width - 1.;

                draw_rectangle(
                    c as f32 * cell_width + 0.5,
                    r as f32 * cell_width + 0.5,
                    cell_size,
                    cell_size,
                    cell.color(),
                );
            }
        }

        let mut text_y = 25.;
        for instruction in &INSTRUCTIONS {
            draw_text(instruction, TEXT_PADDING, text_y, FONT_SIZE, FONT_COLOR);
            text_y += FONT_SIZE + 5.;
        }

        draw_text(
            format!("Cells alive: {}", live_cell_count).as_str(),
            TEXT_PADDING,
            text_y,
            FONT_SIZE,
            FONT_COLOR,
        );

        next_frame().await
    }
}
