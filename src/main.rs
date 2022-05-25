mod state_repr;
mod result;
mod utils;
mod action;
mod interface;
mod terminal;

use interface::cli;


/// Main function of the file
/// 
/// # Parameters
/// 
/// None
/// 
fn main() {
    cli::start_game_on_cli();
}
