use rurel::AgentTrainer;
// agent
use rurel::mdp::Agent;
use rurel::mdp::State;
use rurel::strategy::explore::RandomExploration;
use rurel::strategy::learn::QLearning;
use rurel::strategy::terminate::FixedIterations;

use crate::{SimulationState, BOARD_SIZE};

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub enum AgentAction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Clone, Eq, PartialEq, Hash)]
struct SimState {
    // board: SimulationState,
    // location of the agent on the board
    row: i32,
    column: i32,
    prev_move: Option<AgentAction>,
}

impl SimState {
    fn new() -> Self {
        Self {
            // board: [[CellState::Dead; BOARD_SIZE]; BOARD_SIZE],
            row: 0,
            column: 0,
            prev_move: None,
        }
    }
}

impl State for SimState {
    type A = AgentAction;
    fn reward(&self) -> f64 {
        // // sum all dead cells to encourage growth
        // // TODO: make this more interesting
        // let mut dead_cells = 0;
        //
        // for row in self.board.iter() {
        //     for cell in row.iter() {
        //         if *cell == CellState::Dead {
        //             dead_cells += 1;
        //         }
        //     }
        // }
        //
        // // TODO: need to punish lack of exploration
        //
        // -dead_cells as f64

        // just try to grow to bottom right
        -(((BOARD_SIZE as i32 - self.column).pow(2) + (BOARD_SIZE as i32 - self.row).pow(2)) as f64)
            .sqrt()
        // TODO: need to punish lack of exploration
    }

    fn actions(&self) -> Vec<AgentAction> {
        let mut actions = vec![];

        // let state = &self.board;
        let row = self.row;
        let column = self.column;

        // let left = get_cell_with_bounds_check(state, Some(row), column.checked_sub(1));
        // let right = get_cell_with_bounds_check(state, Some(row), column.checked_add(1));
        // let up = get_cell_with_bounds_check(state, row.checked_sub(1), Some(column));
        // let down = get_cell_with_bounds_check(state, row.checked_add(1), Some(column));

        if column > 0 && self.prev_move != Some(AgentAction::Right) {
            actions.push(AgentAction::Left);
        }
        if column < BOARD_SIZE as i32 - 1 && self.prev_move != Some(AgentAction::Left) {
            actions.push(AgentAction::Right);
        }
        if row > 0 && self.prev_move != Some(AgentAction::Down) {
            actions.push(AgentAction::Up);
        }
        if row < BOARD_SIZE as i32 - 1 && self.prev_move != Some(AgentAction::Up) {
            actions.push(AgentAction::Down);
        }

        actions
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
        // self.state.board[self.state.row][self.state.column] = CellState::Alive;

        // actions are all valid, so this should be safe
        // grow into the desired direction
        match action {
            AgentAction::Left => {
                self.state.column -= 1;
                self.state.prev_move = Some(AgentAction::Left);
            }
            AgentAction::Right => {
                self.state.column += 1;
                self.state.prev_move = Some(AgentAction::Right);
            }
            AgentAction::Up => {
                self.state.row -= 1;
                self.state.prev_move = Some(AgentAction::Up);
            }
            AgentAction::Down => {
                self.state.row += 1;
                self.state.prev_move = Some(AgentAction::Down);
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
        &QLearning::new(0.5, 0.01, 2.),
        &mut FixedIterations::new(100000),
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
            AgentAction::Left,
            AgentAction::Right,
            AgentAction::Up,
            AgentAction::Down,
        ];

        let mut best_action: Option<(AgentAction, f64)> = None;

        let model_state = SimState {
            // board: *simulation_state,
            row: row as i32,
            column: column as i32,
            prev_move,
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
