use crate::action::Action;
use crate::state_repr::{GameElements, GameState, SpecialSkill};
use crate::utils::is_not_out_of_bound;


/// Get the state result from an action to a state
/// 
/// # Parameters
/// 
/// * state: Game state
/// * action: Action done by the player
/// * player: id player, 0 or 1
/// 
/// # Return
/// GameState: Return a new object game state
/// 
pub fn result_after_action(state: &GameState, action: &Action, player: u8) -> GameState {
    let (xp, yp) = state.get_player_position_with_idx(player);

    // We will create a new state cloned from the original one
    let mut new_state = state.clone();

    match action {
        Action::Down => {
            // Go down
            // curent position become Non passable
            new_state.change_elem_on_coor(xp as usize, yp as usize, GameElements::NonPassable);
            new_state.change_player_pos(player, xp as usize, (yp + 1) as usize);
            change_game_state_after_move(&mut new_state, player);
        }
        Action::Up => {
            // Go down
            new_state.change_elem_on_coor(xp as usize, yp as usize, GameElements::NonPassable);
            new_state.change_player_pos(player, xp as usize, (yp - 1) as usize);
            change_game_state_after_move(&mut new_state, player);
        }
        Action::Left => {
            // Go Left
            new_state.change_elem_on_coor(xp as usize, yp as usize, GameElements::NonPassable);
            new_state.change_player_pos(player, (xp - 1) as usize, yp as usize);
            change_game_state_after_move(&mut new_state, player);
        }
        Action::Right => {
            // Go Right TODO refactor repetition
            new_state.change_elem_on_coor(xp as usize, yp as usize, GameElements::NonPassable);
            new_state.change_player_pos(player, (xp + 1) as usize, yp as usize);
            change_game_state_after_move(&mut new_state, player);
        }
        Action::SpSkill => {
            // use special skill! Does not move
            use_sp_skill(&mut new_state, player);
        }
    }
    new_state
}

/// Get unpassable when doing clear road special skill
///
///
fn get_unpassable_on_clear_road_sp_skill(state: &GameState, xp: i8, yp: i8) -> Vec<(i8, i8)> {
    // check Up (check if < 0)
    let tuple_check = [(xp, yp - 1), (xp, yp + 1), (xp + 1, yp), (xp - 1, yp)];
    let mut tuple_filtered = Vec::new();
    for coor in tuple_check {
        if is_not_out_of_bound(coor.0, coor.1, state.get_height_width() as i8) {
            match state.get_elem_on_coor(coor.0 as usize, coor.1 as usize) {
                GameElements::NonPassable => {
                    tuple_filtered.push(coor);
                }
                _ => (),
            }
        }
    }
    return tuple_filtered;
}

/// Result function on using a special skill
/// Update the state directly
/// 
/// # Parameters
/// 
/// * state: Mutable Game State to change it.
/// * player: Player ID
/// 
fn use_sp_skill(state: &mut GameState, player: u8) {
    let (xp, yp) = state.get_player_position_with_idx(player);
    let sp_skill = state.get_player_sp(player);
    let opposite_player = 1 - player;

    match sp_skill {
        SpecialSkill::ClearRoadTile => {
            // Clear NonPassable and becomes passable
            // check up, down, left, right
            let coor_not_passable = get_unpassable_on_clear_road_sp_skill(&state, xp as i8, yp as i8);
            for coor in coor_not_passable { 
                // change the state to passable
                state.change_elem_on_coor(coor.0 as usize, coor.1 as usize, GameElements::Passable);
            }
        }
        SpecialSkill::ReduceEnemiesATK => {
            state.change_player_atk(opposite_player, -1);
        }
        _ => (),
    };
    state.set_player_sp(player, SpecialSkill::None);
}

///
/// Change Game state (like stepping on bonuses) after move.
///
fn change_game_state_after_move(state: &mut GameState, player: u8) {
    let (xp, yp) = state.get_player_position_with_idx(player);
    let game_element_on_coor = state.get_map_representation()[xp as usize][yp as usize];

    match game_element_on_coor {
        GameElements::BonusATKBonusTile => {
            state.change_player_atk(player, 1);
        }
        GameElements::MinusATKBonusSpTile => {
            state.set_player_sp(player, SpecialSkill::ReduceEnemiesATK);
        }
        GameElements::ClearRoadBonusSpTile => {
            state.set_player_sp(player, SpecialSkill::ClearRoadTile);
        }
        _ => (),
    }
}
