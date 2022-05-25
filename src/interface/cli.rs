use crate::action::{Action, get_legal_action};
use crate::result::result_function::result_after_action;
use crate::state_repr::{GameState, MapRandom};
use crate::terminal::{get_point_for_player_on_terminal, is_terminal_state};


/// GAME MAP SIZE
const MAP_SIZE: usize = 5;

///
/// Translate move input by the user to its Action
/// 
/// # Parameters
///
/// * user_inp String of user
/// 
/// # Returns
/// 
/// None or Action
///
fn translate_move(user_inp: &String) -> Option<Action> {
    match user_inp.trim() {
        "u" => Some(Action::Up),
        "l" => Some(Action::Left),
        "r" => Some(Action::Right),
        "d" => Some(Action::Down),
        "s" => Some(Action::SpSkill),
        _ => None,
    }
}

/// Start the game on CLI
pub fn start_game_on_cli() {
    println!("You'll act!");

    let mut state: GameState = GameState::new(MAP_SIZE, MapRandom::FullyRandom);
    // Check possible move
    let mut action = String::new();
    let mut player_turn = 0;
    let mut count_turn = 1_i32;

    while action.trim() != "q" {

        // Check terminal first
        if is_terminal_state(&state) {
            let p_p1 = get_point_for_player_on_terminal(&state, true);
            if p_p1 > 0 {
                println!("PLAYER 1 WIN!");
            } else if p_p1 < 0 {
                println!("PLAYER 1 LOSE!");
            } else {
                println!("DRAWWW!!")
            }
            break;
        }

        action = "".to_string();
        println!("PLAYER {} | COUNT TURN {}", player_turn + 1, count_turn);
        println!("====");
        println!("Movement: \nu: up \nl: left \nr: right \nd: down \ns: special skill");
        println!("====");
        state.print_pretty_state();
        let legal_action = get_legal_action(&state, player_turn);

        if legal_action.len() > 0 {
            // Check legal action 
            println!("Possible Action: {:#?}", legal_action);
            println!("YOUR MOVE : ");
            std::io::stdin()
                .read_line(&mut action)
                .expect("Failed to read message");
            match translate_move(&action) {
                None => {
                    if action.trim() == "q" {
                        println!("You Quitted!")
                    } else {
                        println!("Your inp {action} is wrong! put it again!")
                    }
                }
                Some(act) => {
                    // check if act is valid, if not ask user to inp!
                    if legal_action.contains(&act) {
                        let new_state = result_after_action(&state, &act, player_turn);
                        // check terminal
                        state = new_state;
                        count_turn += 1;
                        player_turn = 1 - player_turn;
                    }
                    else {
                        println!("INVALID MOVE FOR {:#?}, CHOOSE AGAIN!", act);
                    }

                }
            }
        }
        else {
            // No valid move, SKIP turn
            println!("YOU DONT HAVE ANY MOVEMENT! SKIPPED");
            state = state;
            count_turn += 1;
            player_turn = 1 - player_turn;
        }
    }
}
