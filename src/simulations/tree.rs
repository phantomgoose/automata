use crate::models::trainer::{AgentAction, QModel};
use crate::{AgentColumn, AgentRow, CellState, SimulationState, BOARD_SIZE};

// Grow in a tree-like pattern, in theory
pub fn get_tree_next_cell_state(
    state: &SimulationState,
    buffer: &mut SimulationState,
    row: usize,
    column: usize,
    model: &QModel,
    agent_row: &mut AgentRow,
    agent_column: &mut AgentColumn,
    prev_move: &mut Option<AgentAction>,
) {
    // ignore all cells besides the one the agent is on atm
    if row != *agent_row || column != *agent_column {
        return;
    }

    // for the cell that the agent is on, move the agent and mark current cell as live
    let action = model.get_next_action(state, *agent_row, *agent_column, prev_move.clone());

    if let Some(action) = action {
        match action {
            AgentAction::ToTrunk => {
                buffer[*agent_row][*agent_column] = CellState::Trunk;
                *agent_row -= 1;
            }
            AgentAction::SproutLeaves => {
                buffer[*agent_row][*agent_column - 1] = CellState::Leaf;
                buffer[*agent_row][*agent_column] = CellState::Leaf;
                buffer[*agent_row][*agent_column + 1] = CellState::Leaf;
            }
            AgentAction::DoNothing => {
                // no-op
            }
        }
    }
}
