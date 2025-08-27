use crate::content::board::Board;

pub struct Game{
    step: i32,
    current_player: i32,
    black_score: i32,
    white_score: i32,
    game_tip: String,
    board: Board,
    
}