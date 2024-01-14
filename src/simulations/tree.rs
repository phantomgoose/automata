use crate::models::trainer::{AgentAction, QModel};
use crate::{AgentColumn, AgentRow, CellState, SimulationState, BOARD_SIZE};

// Grow in a tree-like pattern, in theory
pub fn get_tree_next_cell_state(
    state: &SimulationState,
    row: usize,
    column: usize,
    model: &QModel,
    agent_row: &mut AgentRow,
    agent_column: &mut AgentColumn,
    prev_move: &mut Option<AgentAction>,
) -> CellState {
    // ignore all cells besides the one the agent is on atm
    if row != *agent_row || column != *agent_column {
        return state[row][column];
    }

    // for the cell that the agent is on, move the agent and mark current cell as live
    let action = model.get_next_action(state, *agent_row, *agent_column, prev_move.clone());

    if let Some(action) = action {
        match action {
            AgentAction::Left => *agent_column -= 1,
            AgentAction::Right => *agent_column += 1,
            AgentAction::Up => *agent_row -= 1,
            AgentAction::Down => *agent_row += 1,
        }
        *prev_move = Some(action);
        return CellState::Alive;
    }

    CellState::Dead
}
