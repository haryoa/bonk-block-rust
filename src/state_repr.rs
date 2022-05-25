//! State Representation Module
//!
//! # WTF is this?
//!
//! This is a module that respresents a state of the game
//! Any state related things goes here
//!
//!
use rand::Rng;
use strum::EnumCount;
use strum_macros::{Display, EnumCount as EnumCountMacro, EnumIter};

///
/// Enum for special skill .
/// TODO move out
///
#[derive(PartialEq, Debug, Clone)]
pub enum SpecialSkill {
    None,
    ReduceEnemiesATK,
    ClearRoadTile,
}

/// Map Randomness Setting enum
#[derive(PartialEq, Debug)]
pub enum MapRandom {
    FullyRandom,
}

/// Game Elements in the game
#[derive(Debug, Clone, Copy, PartialEq, EnumCountMacro, EnumIter, Display)]
pub enum GameElements {
    Passable,
    NonPassable,
    PlayerOne,
    PlayerTwo,
    BonusATKBonusTile,
    MinusATKBonusSpTile,
    ClearRoadBonusSpTile,
}

/// This struct represents the game state
///
/// # State representation
///
/// It contains a 2D array. Each array may contains:
/// 'o': passable tile
/// 'x': non-passable tile
/// '1': first player
/// '2': second player
/// '+': Bonus ATK +1
/// '-': Minus ATK for enemy -1
/// 'c': Clear Road for all direction
///
#[derive(Debug, Clone)]
pub struct GameState {
    height_width: usize,
    map_representation: Vec<Vec<GameElements>>,
    first_player_pos: (usize, usize),
    second_player_pos: (usize, usize),
    first_player_sp: SpecialSkill,
    second_player_sp: SpecialSkill,
    first_player_atk: i16,
    second_player_atk: i16,
}

/// Useful for printing
///
/// # Parameters
///
/// * game_elem: Any Game elements struct
///
/// # Returns
///
/// Char thar represent the game element
fn transform_enum_elem_to_char(game_elem: &GameElements) -> char {
    match game_elem {
        GameElements::BonusATKBonusTile => '+',
        GameElements::ClearRoadBonusSpTile => 'c',
        GameElements::MinusATKBonusSpTile => '-',
        GameElements::NonPassable => 'x',
        GameElements::Passable => 'o',
        GameElements::PlayerOne => '1',
        GameElements::PlayerTwo => '2',
    }
}

impl GameState {
    /// Change state of the board game in x, y, and value
    ///
    /// # Parameters
    ///
    /// * x: coordinate x  
    /// * y: coordinate y  
    /// * val: value that want to be replaced there
    ///
    pub fn change_elem_on_coor(&mut self, x: usize, y: usize, val: GameElements) {
        self.map_representation[x][y] = val;
    }

    pub fn get_elem_on_coor(&self, x: usize, y: usize) -> &GameElements {
        return &self.map_representation[x][y];
    }

    /// get player special skill
    ///
    /// # Parameters
    /// player: id of player, possible choices: 0, 1
    pub fn get_player_sp(&self, player: u8) -> &SpecialSkill {
        if player == 0 {
            &self.first_player_sp
        } else {
            &self.second_player_sp
        }
    }

    pub fn get_player_position(&self) -> ((usize, usize), (usize, usize)) {
        (self.first_player_pos, self.second_player_pos)
    }

    pub fn get_player_position_with_idx(&self, player: u8) -> (u8, u8) {
        if player == 0 {
            (self.first_player_pos.0 as u8, self.first_player_pos.1 as u8)
        } else {
            (
                self.second_player_pos.0 as u8,
                self.second_player_pos.1 as u8,
            )
        }
    }

    /// Getter of height_width
    pub fn get_height_width(&self) -> usize {
        self.height_width
    }

    pub fn get_player_atk(&self, is_player_one: bool) -> i16 {
        if is_player_one {
            self.first_player_atk
        } else {
            self.second_player_atk
        }
    }

    pub fn set_player_sp(&mut self, player: u8, special: SpecialSkill) {
        if player == 0 {
            self.first_player_sp = special;
        } else {
            self.second_player_sp = special;
        }
    }

    pub fn change_player_atk(&mut self, player: u8, point_inc: i16) {
        if player == 0 {
            let new = self.first_player_atk + point_inc;
            self.first_player_atk = new;
        } else {
            let new = self.second_player_atk + point_inc;
            self.second_player_atk = new;
        };
    }
    /// Getter of map
    pub fn get_map_representation(&self) -> &Vec<Vec<GameElements>> {
        &self.map_representation
    }

    /// Print map
    /// Remember coordinates is x,y not y,x
    /// so start looping from the first index
    /// We use first index
    pub fn print_map(&self) {
        let ((x1, y1), (x2, y2)) = self.get_player_position();
        for y in 0..self.height_width {
            let mut collect_str: String = "".to_string();
            for x in 0..self.height_width {
                // COLLIDE!
                if x1 == x && x2 == x && y1 == y && y2 == y {
                    collect_str.push('V');
                    collect_str.push(' ');
                } else if x1 == x && y1 == y {
                    collect_str.push('1');
                    collect_str.push(' ');
                } else if x2 == x && y2 == y {
                    collect_str.push('2');
                    collect_str.push(' ');
                } else {
                    collect_str.push(transform_enum_elem_to_char(&self.map_representation[x][y]));
                    collect_str.push(' ');
                }
            }
            println!("{}", collect_str);
        }
    }

    /// print state pretty
    pub fn print_pretty_state(&self) {
        println!(
            "ATK P1 = {} \t ATK P2 = {}",
            self.first_player_atk, self.second_player_atk
        );
        println!(
            "SP SKILL P1 = {:#?} \t SP SKILL P2 = {:#?}",
            &self.first_player_sp, &self.second_player_sp
        );
        self.print_map();
        println!("---")
    }

    /// Change player position to new pos_x and new_pos_y
    pub fn change_player_pos(&mut self, player: u8, new_pos_x: usize, new_pos_y: usize) {
        let mut player = if player == 0 {
            &mut self.first_player_pos
        } else {
            &mut self.second_player_pos
        };
        player.0 = new_pos_x;
        player.1 = new_pos_y;
    }

    ///
    /// Instantiate a GameState Struct
    ///
    /// # Parameters
    /// height_width: Height and Width of the area
    ///
    pub fn new(height_width: usize, map_randomness: MapRandom) -> GameState {
        let map_repr = create_map_representation(height_width, map_randomness);
        GameState {
            height_width: height_width,
            map_representation: map_repr.0,
            first_player_pos: map_repr.1,
            second_player_pos: map_repr.2,
            first_player_sp: SpecialSkill::None,
            second_player_sp: SpecialSkill::None,
            first_player_atk: 0,
            second_player_atk: 0,
        }
    }
}

///
/// Create map representation of the game itself
/// TODO: add more **Compact MAP**
///
/// # Returns
/// Return the vector creation
///
fn create_map_representation(
    height_width: usize,
    map_randomness: MapRandom,
) -> (Vec<Vec<GameElements>>, (usize, usize), (usize, usize)) {
    // Placeholder
    let mut vec = vec![vec![GameElements::Passable; height_width]; height_width];
    let x_mid = f32::from(i8::try_from(height_width).unwrap()) / 2.0;
    let player_mid_location = x_mid.ceil() as i8 as usize;
    let player_one_coor = (player_mid_location - 1, 0); // x, y
    let player_two_coor = (player_mid_location - 1, height_width - 1);

    // creater randomness on the map based on the MapRandomChoice
    if map_randomness == MapRandom::FullyRandom {
        // Fully Random : Just randomm!!
        // end of game element of PASSABLE chance
        let end_ge = GameElements::COUNT - 3;
        for i in 0..vec.len() {
            for j in 0..vec[i].len() {
                // if position is the player, SKIP!
                if (i == player_one_coor.0 && j == player_one_coor.1)
                    || (i == player_two_coor.0 && j == player_two_coor.1)
                {
                    continue;
                }
                let mut rng = rand::thread_rng();
                let random_num = rng.gen_range(0..10);
                // let choosen_elem: GameElements;
                let choosen_elem = if random_num <= end_ge {
                    GameElements::Passable
                } else if random_num > end_ge + 1 && random_num < end_ge + 3 {
                    GameElements::BonusATKBonusTile
                } else if random_num == end_ge + 3 {
                    GameElements::ClearRoadBonusSpTile
                } else if random_num == end_ge + 4 {
                    GameElements::MinusATKBonusSpTile
                } else {
                    GameElements::NonPassable
                };
                vec[i][j] = choosen_elem;
            }
        }

        // Loop all of them and put it randomly!
        // Make passable has higher chance than the others
    }
    (vec, player_one_coor, player_two_coor)
}
