use rurel::AgentTrainer;

use rurel::mdp::Agent;
use rurel::mdp::State;
use rurel::strategy::explore::RandomExploration;
use rurel::strategy::learn::QLearning;
use rurel::strategy::terminate::FixedIterations;

use crate::models::cell_to_u8;
use crate::util::get_cell_with_bounds_check;
use crate::{CellState, SimulationState, BOARD_SIZE};

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub enum AgentAction {
    ToTrunk,      // turn current cell to trunk and move up
    SproutLeaves, // grow leaves around current location
    DoNothing,    // no-op
}

#[derive(Clone, Eq, PartialEq, Hash)]
struct SimState {
    board: SimulationState,
    left: Option<CellState>,
    right: Option<CellState>,
    down: Option<CellState>,
    current: CellState,
    row: usize,
    column: usize,
}

impl SimState {
    fn new() -> Self {
        Self {
            board: [[CellState::Empty; BOARD_SIZE]; BOARD_SIZE],
            left: Some(CellState::Empty),
            right: Some(CellState::Empty),
            down: None,
            current: CellState::Empty,
            row: BOARD_SIZE - 1,
            column: BOARD_SIZE / 2,
        }
    }
}

impl State for SimState {
    type A = AgentAction;
    fn reward(&self) -> f64 {
        let mut leaf_count = 0;
        let mut trunk_count = 0;

        for cell in self.board.iter().flatten() {
            if *cell == CellState::Trunk {
                trunk_count += 1;
            } else if *cell == CellState::Leaf {
                leaf_count += 1;
            }
        }

        // // sun is in the upper right
        // let (sun_pos_x, sun_pos_y) = (BOARD_SIZE as i32 * 2, -(BOARD_SIZE as i32));
        //
        // // reward leaves more the closer they are to the sun
        // // reward the trunk at a diminished rate
        // // TODO: this is wrong as it doesn't reward the location of each leaf, just the current cell
        // (trunk_count / 100 + leaf_count) as f64
        //     * -(((sun_pos_x - self.column as i32).pow(2) + (sun_pos_y - self.row as i32).pow(2))
        //         as f64)
        //         .sqrt()

        (trunk_count / 100 + leaf_count) as f64
    }

    fn actions(&self) -> Vec<AgentAction> {
        let mut valid_actions = vec![AgentAction::DoNothing];

        // can turn to trunk:
        if
        // 1) if we're on the ground layer and neighbor cells are free
        (self.row == BOARD_SIZE - 1
            && self.left == Some(CellState::Empty)
            && self.right == Some(CellState::Empty))
            // 2) OR if the cell below is also a trunk AND we're not at the top yet
            || (self.row > 0 && self.down == Some(CellState::Trunk))
        {
            valid_actions.push(AgentAction::ToTrunk);
        }

        // can sprout leaves if the cell below is trunk and height > half of board and we aren't already on a leaf cell
        if self.down == Some(CellState::Trunk) && self.current != CellState::Leaf {
            valid_actions.push(AgentAction::SproutLeaves);
        }

        valid_actions
    }
}

struct TreeAgent {
    state: SimState,
}

impl Agent<SimState> for TreeAgent {
    fn current_state(&self) -> &SimState {
        &self.state
    }

    fn take_action(&mut self, action: &AgentAction) {
        match action {
            AgentAction::ToTrunk => {
                let SimState {
                    mut row,
                    column,
                    mut board,
                    ..
                } = self.state;

                board[row][column] = CellState::Trunk;
                row -= 1;

                let current = get_cell_with_bounds_check(&board, Some(row), Some(column))
                    .expect("current cell should always exist and be valid");
                let left = get_cell_with_bounds_check(&board, Some(row), column.checked_sub(1));
                let right = get_cell_with_bounds_check(&board, Some(row), column.checked_add(1));
                let down = get_cell_with_bounds_check(&board, row.checked_add(1), Some(column));

                self.state = SimState {
                    board,
                    left,
                    right,
                    down,
                    current,
                    row,
                    column,
                };
            }
            AgentAction::SproutLeaves => {
                let SimState { row, column, .. } = self.state;

                if column > 0 {
                    self.state.board[row][column - 1] = CellState::Leaf;
                    self.state.left = Some(CellState::Leaf);
                }

                if column < BOARD_SIZE - 1 {
                    self.state.board[row][column + 1] = CellState::Leaf;
                    self.state.right = Some(CellState::Leaf);
                }

                self.state.board[row][column] = CellState::Leaf;
                self.state.current = CellState::Leaf;
            }
            AgentAction::DoNothing => {
                // no-op
            }
        }
    }
}

fn train() -> AgentTrainer<SimState> {
    let mut trainer = AgentTrainer::new();
    let mut agent = TreeAgent {
        state: SimState::new(),
    };
    trainer.train(
        &mut agent,
        &QLearning::new(0.2, 0.01, -10.),
        &mut FixedIterations::new(100_000),
        &RandomExploration::new(),
    );

    trainer
}

pub struct QModel {
    trainer: AgentTrainer<SimState>,
}

impl QModel {
    pub fn new() -> Self {
        Self { trainer: train() }
    }

    fn get_expected_value(self: &QModel, state: &SimState, action: &AgentAction) -> Option<f64> {
        self.trainer.expected_value(state, action)
    }

    pub fn get_next_action(
        self: &QModel,
        simulation_state: &SimulationState,
        row: usize,
        column: usize,
        prev_move: Option<AgentAction>,
    ) -> Option<AgentAction> {
        let possible_actions = vec![
            // AgentAction::DoNothing,
            AgentAction::SproutLeaves,
            AgentAction::ToTrunk,
        ];

        let mut best_action: Option<(AgentAction, f64)> = None;

        let model_state = SimState {
            board: *simulation_state,
            left: get_cell_with_bounds_check(simulation_state, Some(row), column.checked_sub(1)),
            right: get_cell_with_bounds_check(simulation_state, Some(row), column.checked_add(1)),
            down: get_cell_with_bounds_check(simulation_state, row.checked_add(1), Some(column)),
            current: get_cell_with_bounds_check(simulation_state, Some(row), Some(column))
                .expect("current should be a valid cell"),
            row,
            column,
        };

        for possible_action in possible_actions {
            let weight = self.get_expected_value(&model_state, &possible_action);
            // TODO: refactor
            if let Some(weight) = weight {
                if best_action.is_none() || weight > best_action.clone().unwrap().1 {
                    best_action = Some((possible_action, weight))
                }
            }
        }

        best_action.map(|a| a.0)
    }
}
