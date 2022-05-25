use crate::action::get_legal_action;
use crate::state_repr::GameState;


/// Check if the game ended
/// 
/// # Criteria
/// 
/// This is the criteria how the game ended
/// 1. No move left for both of players (stuck / trapped)
/// 2. Player 1 and Player 2 collided
/// 
/// # Parameters
/// 
/// * state: The current state
/// 
pub fn is_terminal_state(state: &GameState) -> bool {
    let p1_actions = get_legal_action(&state, 0);
    let p2_actions = get_legal_action(&state, 1);

    let ((x1,y1), (x2,y2)) = state.get_player_position();
    
    // condition 1
    if p1_actions.len() == 0 && p2_actions.len() == 0 {
        return true;
    }
    
    if x1 == x2 && y1 == y2 {
        return true;
    }

    return false;
}


/// Decide winner score here
/// 
/// # Parameters
/// 
/// * state: The state
/// * is_player_one: If the score that want to be checked is player one, set it to true
/// 
/// # Returns
/// 
/// Return the state of the winner!
/// 
pub fn get_point_for_player_on_terminal(state: &GameState, is_player_one: bool) -> i16 {
    println!("{:#?}", state);
    if state.get_player_atk(is_player_one) > state.get_player_atk(!is_player_one) {
        // WIN
        100
    }
    else if state.get_player_atk(is_player_one) < state.get_player_atk(!is_player_one) {
        // LOSE
        -100
    }
    else { 0 } // DRAW
}