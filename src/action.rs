use crate::state_repr::{GameElements, GameState, SpecialSkill};

/// Action available for action
#[derive(Debug, PartialEq)]
pub enum Action {
    Up,
    Down,
    Right,
    Left,
    SpSkill,
}

/// Get possible action that can be used given the state
/// for the player
///
/// # Parameters
///
/// * state: the state of the game
/// * player: Player, 0 or 1
///
/// # Returns
/// Return possible action in Vec<Action>
pub fn get_legal_action(state: &GameState, player: u8) -> Vec<Action> {
    let mut legal_vecs: Vec<Action> = Vec::new();
    // contains player 0, player 1
    // coordinates x_player, y_player
    let (x_p, y_p): (i8, i8) = if player == 0 {
        let pos = state.get_player_position().0;
        (pos.0 as i8, pos.1 as i8)
    } else {
        let pos = state.get_player_position().1;
        (pos.0 as i8, pos.1 as i8)
    };

    let current_sp_skill = state.get_player_sp(player);
    let maprepr = state.get_map_representation();
    let len_map = maprepr.len() as i8;

    // validate first x_player, y_player
    // since the size is casted from usize, impossible to get < 0>
    if x_p >= len_map || y_p >= len_map {
        panic!("Invalid player position");
    }

    // Validate Up
    if validate_coor_move(x_p, y_p - 1, maprepr) {
        legal_vecs.push(Action::Up);
    }

    // Validate Down
    if validate_coor_move(x_p, y_p + 1, maprepr) {
        legal_vecs.push(Action::Down);
    }

    // Validate Right
    if validate_coor_move(x_p + 1, y_p, maprepr) {
        legal_vecs.push(Action::Right);
    }

    // Validate Left
    if validate_coor_move(x_p - 1, y_p, maprepr) {
        legal_vecs.push(Action::Left);
    }

    if !matches!(current_sp_skill, SpecialSkill::None) {
        legal_vecs.push(Action::SpSkill);
    }

    return legal_vecs;
}

/// validate action after move (lazy operation)
///
/// # Parameters
/// 
/// * x: x after move
/// * y: y after move
/// * map_repr: map representation (non mutable). Must be square area
///
fn validate_coor_move(x: i8, y: i8, map_repr: &Vec<Vec<GameElements>>) -> bool {
    let valid = x >= 0
    && y >= 0
    && (x as usize) < (map_repr.len() as usize)
    && (y as usize) < (map_repr.len() as usize)
    && map_repr[x as usize][y as usize] != GameElements::NonPassable;
    valid
}
