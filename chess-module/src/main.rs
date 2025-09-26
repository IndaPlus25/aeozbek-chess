use std::{f32::consts::E, iter, thread::current};

struct Piece 
{
        position: String,
        color: String,
        what_type: String,
        has_moved: Option<bool>, // This is for checking if rooks or kings have moved before castling and if en passant is still available to do for pawns
        has_moved_double: Option<bool> // For checking en_passaint we have to know if a pawn moved double steps or not and this variable gives us that option
}

impl Piece 
{
    fn new(position: &str, color: &str, what_type: &str, has_moved: Option<bool>, has_moved_double: Option<bool>) -> Self 
    {
        Piece { position: position.to_string(), color: color.to_string(), what_type: what_type.to_string(), has_moved: has_moved, has_moved_double: has_moved_double}
    }
}

enum GameState 
{
    InProgress,
    Check,
    GameOver,
    Checkmate,
    DeadPosition,
}

struct Game 
{
    gameboard: [[Option<Piece>; 8]; 8],
    current_state: GameState,
    white_king_position: String, // The gamestate is almost completely determined by if the kings are target of another opposite piece or not. In order to increase effectivity we can store the positions of the kings so that we do not have to go through the whole gameboard once again
    black_king_position: String,
    whose_turn: String,
    promotion_type: String // What I understood from instructions is that we will just decide one type that the pawns will promote to and every single pawn will promote to that? Because we do not choose any specific pawn in the set_promotion function so that is how I chose to implement. 
}

impl Game 
{
    pub fn set_promotion(&mut self, piece: String) -> () {

        self.promotion_type = match piece.as_str(){
            "Rook" => "Rook".to_string(),
            "Bishop" => "Bishop".to_string(),
            "Knight" => "Knight".to_string(),
            "Queen" => "Queen".to_string(),
            _ => self.promotion_type.clone()
        } 
    }
    pub fn get_game_state(&self) -> GameState {

        let current_gamestate = self.current_state;
        current_gamestate
    }

    pub fn make_move(&mut self, from: String, to: String) -> Option<GameState> {
        if self.current_state == GameState::InProgress || self.current_state == GameState::Check { // if it is a check the player still has the opportunity to save their king
            
            let mut is_check_white: bool = false; // this boolean variable is for checking if the kings position is under threat of an enemy piece. If both the position of the king is under threat and the king has nowhere to go it is a checkmate, otherwise if kings position is not under threat it means that it is a stalemate
            let mut is_check_black: bool = false; // the reason we specify colors of the check variables is to not let the player do another move that does not save the king from the threat
            let valid_moves = self.get_possible_moves(from); // we shall get the valid moves of the wished piece so that we know where it is able to go
            let [row, column] = reverse_position_converter(from); 
            let [target_row, target_column] = reverse_position_converter(&to);
            let which_piece = self.gameboard[row as usize][column as usize];

            if let Some(chosen_piece) = which_piece {

                if let Some(valid_moves) = valid_moves {
                    
                    if valid_moves.contains(&to) && self.whose_turn == which_piece.color  { // if the user has really chosen a piece and the piece is able to move to that chosen position it is valid

                        chosen_piece.position = to;
                        self.gameboard[target_row as usize][target_column as usize] = Some(chosen_piece.clone());
                        self.gameboard[row as usize][column as usize] = None;
                        let will_a_pawn_be_removed: Option<String> = is_move_en_passant(self.gameboard, chosen_piece, to);
                        if chosen_piece.what_type == "Pawn" {

                            match will_a_pawn_be_removed {

                                Some(pawn_to_be_removed) => { 
                                    let [pawns_row, pawns_column] = pawn_to_be_removed;
                                    self.gameboard[pawns_row as usize][pawns_column as usize] = None;
                                }

                                None => ()
                            }

                            if (target_row == 7 && self.whose_turn == "White") || (target_row == 0 && self.whose_turn == "Black") {
                                
                                current_chosen_piece = self.gameboard[target_row as usize][target_column as usize];
                                match current_chosen_piece {
                                    Some(ref mut piece) => {piece.what_type = self.promotion_type.clone()}
                                    None => ()
                                }
                            }
                        }

                        let white_king_moves_option: Vec<String> = self.get_possible_moves(self.white_king_position); // These vectors will be used to check if other opposite pieces block all the way the kings can go to and hence will help us determining if the game is a checkmate, stalemate or something else
                        let black_king_moves_option: Vec<String> = self.get_possible_moves(self.black_king_position);

                        let white_king_moves: Vec<String> = white_king_moves_option.unwrap();
                        let white_king_moves: Vec<String> = black_king_moves_option.unwrap();
                        
                        for every_row in 0..8 {

                            for every_column in 0..8 {

                                let chosen_piece = self.gameboard[every_row as usize][every_column as usize];

                                if self.whose_turn == "White" && chosen_piece.color == "Black" {

                                    let possible_moves_of_the_enemy = self.get_possible_moves(position_converter(every_row, every_column));
                                    if possible_moves_of_the_enemy.contains(self.white_king_position) {

                                        is_check_white = true;
                                    }

                                else if self.whose_turn == "Black" && chosen_piece.color == "White" {

                                    let possible_moves_of_the_enemy = self.get_possible_moves(position_converter(every_row, every_column));
                                    if possible_moves_of_the_enemy.contains(self.white_king_position) {

                                        is_check_black = true;
                                    }

                                }

                                if self.whose_turn== "White" && is_check_white == true { // if the player's king is under threat it shall be invalid

                                    None
                                }

                                else if self.whose_turn == "Black" && is_check_black {

                                    None
                                }

                            }
                        }

                        if self.whose_turn == "White" {

                            if is_check_white {

                                None
                            }

                            if chosen_piece.what_type == "King" {
                                self.white_king_position = to;
                            }

                            if is_check_black && black_king_moves.len() == 0 {

                                self.current_state = &GameState::Checkmate;
                            }

                            else if is_check_black && black_king_moves.len() > 0 {

                                self.current_state = &GameState::Check;
                            }

                            else if !is_check_black && black_king_moves.len() == 0 {

                                self.current_state = &GameState::InProgress;
                            }

                            else {

                                self.current_state = &GameState::InProgress;
                            }

                            self.whose_turn = "Black";
                            Some(self.current_state)
                        }

                        if self.whose_turn == "Black" {

                            if is_check_black {

                                None
                            }

                            if chosen_piece.what_type == "King" {
                                self.black_king_position = to;
                            }

                            if is_check_white && white_king_moves.len() == 0 {

                                self.current_state = &GameState::Checkmate;
                            }

                            else if is_check_white && white_king_moves.len() > 0 {

                                self.current_state = &GameState::Check;
                            }

                            else if !is_check_white && white_king_moves.len() == 0 {

                                self.current_state = &GameState::InProgress;
                            }

                            else {

                                self.current_state = &GameState::InProgress;
                            }
                            self.whose_turn = "White";
                            Some(self.current_state)
                        }
                    }
                }
            }
        }
    
        pub fn get_possible_moves(&self, position: String) -> Option<Vec<String>>{

            let possible_maximum_range: Option<Vec<String>> = get_maximal_range(self.gameboard, position, &self);

            match possible_maximum_range {

                Some(maximal_ranges) => {for moves in maximal_ranges {

                    let new_gameboard_to_be_scanned = self.gameboard.clone();
                    let [row, column] = reverse_position_converter(position);
                    let chosen_piece: Piece = self.gameboard[row as usize][column as usize].clone();

                    for every_move in possible_maximum_range {

                        let [target_row, target_column] = reverse_position_converter(every_move);
                        new_gameboard_to_be_scanned[target_row as usize][target_column as usize] = chosen_piece;
                        new_gameboard_to_be_scanned[row as usize][column as usize] = None;

                        if chosen_piece.what_type == "Pawn" {

                            let will_a_pawn_be_removed = is_move_en_passant(self.gameboard, chosen_piece, every_move);

                            match will_a_pawn_be_removed {

                                Some(pawn_position) => {

                                    let [pawns_row, pawns_column] = reverse_position_converter(pawn_position);
                                    new_gameboard_to_be_scanned[pawns_row as usize][pawns_column as usize] = None
                                }

                                None => ()
                            }

                        }

                        for rows in 0..8 {

                            for columns in 0..8 {

                                if let Some(targeted_piece) = new_gameboard_to_be_scanned[rows][columns] {

                                    if targeted_piece.color != self.whose_turn {

                                        let enemys_move_list = get_maximal_range(new_gameboard_to_be_scanned, position_converter(rows, columns), &self);
                                        
                                        if self.whose_turn == "White" {
                                            if let Some(index) = enemys_move_list.iter().position(|&x| x == self.white_king_position) {
                                                possible_maximum_range.remove(index);
                                            }
                                        }
                                        
                                        else {
                                            if let Some(index) = enemys_move_list.iter().position(|&x| x == self.black_king_positionking_position) {
                                                possible_maximum_range.remove(index);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    Some(possible_maximum_range)
                }
            }

            None => None
        }


    }

}}



    pub fn new() -> Game {

        let mut chessboard: [[Option<Piece>; 8]; 8] = [[None; 8]; 8];

        for column in 0..8
        {

            if column == 0 || column == 7
            {
            
                chessboard[0][column] = Piece::new(&position_converter(0 as i8, column as i8), "Black", "Rook", false, None); 
                chessboard[1][column] = Piece::new(&position_converter(1 as i8, column as i8), "Black", "Pawn", false, false); // At start, in every column there is two pawns and two different pieces on both sides which respectively are black and white. That's why we have the numbers like 0, 1, 6 ,7 (one special piece and pawn at one end and vice versa at the other end of the table)
                chessboard[6][column] = Piece::new(&position_converter(6 as i8, column as i8), "White", "Pawn", false, false);
                chessboard[7][column] = Piece::new(&position_converter(7 as i8, column as i8), "White", "Rook", false,false); //On contrary to rooks, pawns have the variable 'has_mode' on None by default. This variable is just for checking if rooks or kings have moved before castling

            }

            else if column == 1 || column == 6
            {
            
                chessboard[0][column] = Piece::new(&position_converter(0 as i8, column as i8), "Black", "Knight", None, false); 
                chessboard[1][column] = Piece::new(&position_converter(1 as i8, column as i8), "Black", "Pawn", false, false); 
                chessboard[6][column] = Piece::new(&position_converter(6 as i8, column as i8), "White", "Pawn", false, false);
                chessboard[7][column] = Piece::new(&position_converter(7 as i8, column as i8), "White", "Knight", None, false); 
            }

            else if column == 2 || column == 5
            {
            
                chessboard[0][column] = Piece::new(&position_converter(0 as i8, column as i8), "Black", "Bishop", None, false); 
                chessboard[1][column] = Piece::new(&position_converter(1 as i8, column as i8), "Black", "Pawn", false, false); 
                chessboard[6][column] = Piece::new(&position_converter(6 as i8, column as i8), "White", "Pawn", false, false);
                chessboard[7][column] = Piece::new(&position_converter(7 as i8, column as i8), "White", "Bishop", None, false); 
            }

            else if column == 3
            {
            
                chessboard[0][column] = Piece::new(&position_converter(0 as i8, column as i8), "Black", "Queen", None, false); 
                chessboard[1][column] = Piece::new(&position_converter(1 as i8, column as i8), "Black", "Pawn", false, false); 
                chessboard[6][column] = Piece::new(&position_converter(6 as i8, column as i8), "White", "Pawn", false, false);
                chessboard[7][column] = Piece::new(&position_converter(7 as i8, column as i8), "White", "Queen", None, false); 
            }

            else if column == 4
            {
            
                chessboard[0][column] = Piece::new(&position_converter(0 as i8, column as i8), "Black", "King", false, false); 
                chessboard[1][column] = Piece::new(&position_converter(1 as i8, column as i8), "Black", "Pawn", false, false); 
                chessboard[6][column] = Piece::new(&position_converter(6 as i8, column as i8), "White", "Pawn", false, false);
                chessboard[7][column] = Piece::new(&position_converter(7 as i8, column as i8), "White", "King", false, false); 
            }
        }
        
        let new_gameboard: Game = Game { gameboard: chessboard, current_state: InProgress, white_king_position: "E1", black_king_position: "E8", whose_turn: "White", promotion_type: ""  };
        new_gameboard
                    
    }
}

fn position_converter(x: i8, y: i8) -> String 
{

    let row: char = char::from_u32(x as u32 + 65).unwrap(); // 65 is A's ASCII chart number, in Rust chars can also be modified as if they are numbers and there is a special chart (ASCII chart) for that
    let column: char = char::from_u32(y as u32).unwrap();

    let position: String = [row, column].iter().collect();

    position
}

fn reverse_position_converter(position: &str) -> [i8; 2]
{   
    let column = position[0].as_bytes()[0] - 65; 
    let row: i8 = position[1].to_string().parse().unwrap();

    let coordinates: [i8; 2] = [row, column];
    coordinates
}


fn get_opposite_kings_range(gameboard: &mut [[Option<Piece>; 8]; 8], opposite_king: Piece) -> Vec<String>  // The reason of this function's existence is that when two kings can never be side by side. They shall at least have  a distance of one square and as the get_possible_moves is having recursion it would throw out an error if it were to come to the king. 
{
    let maximal_range_of_opposite_king: Vec<String> = Vec::new();
    let [row, column] = reverse_position_converter(opposite_king.position).iter().collect();

    if row > 0
    {   
        is_valid_move_king(gameboard, row - 1, column, chosen_piece, maximal_range_of_king, new_gameboard_to_be_scanned);
        if column > 0
        {
             is_valid_move_king(gameboard, row - 1, column - 1, chosen_piece, maximal_range_of_king, new_gameboard_to_be_scanned); 
        }

        if column < 7
        {
            is_valid_move_king(gameboard, row - 1, column + 1, chosen_piece, maximal_range_of_king, new_gameboard_to_be_scanned);
        }

    }

    if row < 7
    {   
        is_valid_move_king(gameboard, row + 1, column, chosen_piece, maximal_range_of_king, new_gameboard_to_be_scanned);
        } 
        if column > 0
        {
            is_valid_move_king(gameboard, row + 1, column - 1, chosen_piece, maximal_range_of_king, new_gameboard_to_be_scanned);  
        }

        if column < 7
        {
           is_valid_move_king(gameboard, row + 1, column + 1, chosen_piece, maximal_range_of_king, new_gameboard_to_be_scanned);
        }
    if column > 0 {

        is_valid_move_king(gameboard, row , column - 1, chosen_piece, maximal_range_of_king, new_gameboard_to_be_scanned);  
    }
    
    if column < 7
    {
        is_valid_move_king(gameboard, row, column + 1, chosen_piece, maximal_range_of_king, new_gameboard_to_be_scanned);   
    }

    maximal_range_of_opposite_king

}



fn is_empty(gameboard: &[[Option<Piece>; 8]; 8], position: String) -> bool
{   
    let [row, column] = reverse_position_converter(position);
    let current_targeted_piece = &gameboard[row as usize][column as usize];

    match current_targeted_piece {
        Some(piece) => {
            false
        },
        None => {
            true
        }
    }
}


fn is_valid_move_pawn(gameboard: &[[Option<Piece>; 8]; 8], row: i8, column:i8, main_chosen_piece: &Piece, possible_moves: Vec<String>)
{   
    let current_targeted_piece = &gameboard[row as usize][column as usize];

    match current_targeted_piece {

        Some(piece) => {if main_chosen_piece.color != current_targeted_piece.color {
            possible_moves.push(position_converter(row, column));
        }},

        None => {
            ()
        }
    }

}

fn is_valid_en_passant(gameboard: &[[Option<Piece>; 8]; 8], row: i8, column:i8, main_chosen_piece: &Piece, possible_moves: Vec<String>) {

    if main_chosen_piece.color == "Black" {

        let go_forward_constant: i8 = 1; // if a black pawn will go forward it means that it will be placed in an array that is located closer to the end and vice versa for the white

    }

    else {
        
        let go_forward_constant: i8 = -1;
    }

    if column < 7 {
        let piece_at_right: &Option<Piece> = &gameboard[row][column + 1];

        match piece_at_right {
            Some(piece) => {if piece.color != main_chosen_piece.color && piece.has_moved_double {
                let piece_on_diagonal: &Option<Piece> = &gameboard[row + go_forward_constant as usize][column + 1 as usize];
            
                match &piece_on_diagonal {

                    Some(&piece_diagonal) => (), // if let say there is a white piace at the square which is located diagonally, the coordinate of that piece will be put as there is already a check for it in the pawn part of the get_possible_moves
                
                    None => possible_moves.push(position_converter(row + go_forward_constant, column + 1))
                }
            }}

            None => ()

        }
    }

    if column > 0 {
        let piece_at_left: &Option<Piece> = &gameboard[row][column - 1];

        match piece_at_left {

            Some(piece) => {if &piece.color != main_chosen_piece.color && &piece.has_moved_double {
                let piece_on_diagonal: Option<Piece> = &gameboard[row + go_forward_constant][column - 1];
            
                match piece_on_diagonal {

                    Some(piece_diagonal) => (), 
                
                    None => possible_moves.push(position_converter(row + go_forward_constant, column - 1))
                }
            }}

            None => ()

        }
    }


}

fn is_move_en_passant(gameboard: &[[Option<Piece>; 8]; 8], main_chosen_piece: &Piece, target_position: String) -> Option<String>  // If the move is an en passant it will return the position of the pawn that is being captured by the other pawn
{
    let [row, column] = reverse_position_converter(&main_chosen_piece.position);

    if row == 3 && main_chosen_piece.color == "White" {

        let get_forward_constant: i8 = -1;
    }

    else if row == 4 && main_chosen_piece.color == "Black" {

        let get_forward_constant: i8 = 1;

    }

    else {

        None
    }

    if is_empty(&gameboard, target_position) {

        if position_converter(row + get_forward_constant, column - 1) == target_position && !is_empty(&gameboard, position_converter(row, column - 1)) {

            let current_targeted_piece = &gameboard[row][column - 1];

            match current_targeted_piece {

                Some(piece) => {if piece.what_type == "Pawn" && piece.color == main_chosen_piece.color {

                    if piece.has_moved && piece.has_double_moved {

                        reverse_position_converter(piece.position)
                    }
                }}

                None => None
            }
        }

        else if position_converter(row + get_forward_constant, column + 1) == target_position && !is_empty(&gameboard, position_converter(row, column + 1)) {

            let current_targeted_piece = &gameboard[row][column + 1];

            match current_targeted_piece {

                Some(piece) => {if piece.what_type == "Pawn" && piece.color == main_chosen_piece.color {
                    if piece.has_moved && piece.has_double_moved {

                        reverse_position_converter(piece.position)
                    }
                }}

                None => None
            }
        }
    }
    
}

fn is_valid_move_knight(gameboard: &[[Option<Piece>; 8]; 8], row: &i8, column: &i8, main_chosen_piece: &Piece, possible_moves: &mut Vec<String>)
{   
    let current_targeted_piece = &gameboard[row as usize][column as usize];

    match current_targeted_piece {
        Some(piece) => {if main_chosen_piece.color != current_targeted_piece.color {
            possible_moves.push(position_converter(row, column));
        }},
        None => {
            possible_moves.push(position_converter(row, column))
        }
    }

}

fn is_valid_move_bishop(gameboard: &[[Option<Piece>; 8]; 8], row: i8, column:i8, main_chosen_piece: &Piece, mut possible_moves: Vec<String>, check_variable: bool, row_constant_positive: bool, column_constant_positive: bool, loop_variable: i8) {
    
    if row_constant_positive && column_constant_positive {
        let target_row = row + loop_variable;
        let target_column = column + loop_variable;
        let go_back_constant_row = -1;
        let go_back_constant_column = -1;
    }

    else if !row_constant_positive && column_constant_positive {
        let target_row = row - loop_variable;
        let target_column = column + loop_variable;
        let go_back_constant_row = 1;
        let go_back_constant_column = -1;
    }

    else if row_constant_positive && !column_constant_positive {
        let target_row = row + loop_variable;
        let target_column = column - loop_variable;
        let go_back_constant_row = -1;
        let go_back_constant_column = 1;
    }

    else {
        let target_row = row - loop_variable;
        let target_column = column - loop_variable;
        let go_back_constant_row = 1;
        let go_back_constant_column = 1;
    }
    
    let current_targeted_piece: &Option<Piece> = &gameboard[target_row as usize][target_column as usize];

    match current_targeted_piece
    {
        Some(piece) => {if piece.color != chosen_piece.color
        { possible_moves.push(position_converter(target_row, target_column))}

        else 
        {
            possible_moves.push(position_converter(target_row + go_back_constant_row, target_column + go_back_constant_column)); //when there is a piece on the way of bishop it cannot go further. If we do not have this boolean variable the loop might continue to go into this if-block although bishop cannot jumpon pieces.
        }

        check_variable = true;
    }

        None => {
            possible_moves.push(position_converter(target_row, target_column))
        }
    }
}

fn is_valid_move_rook(gameboard: &[[Option<Piece>; 8]; 8], row: i8, column:i8, main_chosen_piece: &Piece, mut possible_moves: Vec<String>, mut check_variable: bool, row_or_column: String, constant_positive: bool, loop_variable: i8) {
    
    if row_or_column.unwrap() == "row"
    {
        if constant_positive
        {
            let target_row = row + loop_variable;
            let target_column = column;
            let go_back_constant_row = -1;
            let go_back_constant_row = 0;

        }
        
        else 
        {
            let target_row = row - loop_variable;
            let target_column = column;
            let go_back_constant_row = 1;
            let go_back_constant_row = 0; 
        }
    }

    else if row_or_column.unwrap() == "column"
    {
        if constant_positive
        {
            let target_row = row;
            let target_column = column + loop_variable;
            let go_back_constant_row = 0;
            let go_back_constant_row = -1;

        }
        
        else 
        {
            let target_row = row;
            let target_column = column - loop_variable; 
            let go_back_constant_row = 0;
            let go_back_constant_row = 1;

        }
    }

    let current_targeted_piece: &Option<Piece> = &gameboard[target_row as usize][target_column as usize];

    match current_targeted_piece
    {
        Some(piece) => {if piece.color != chosen_piece.color
        { possible_moves.push(position_converter(target_row, target_column))}

        else 
        {
            possible_moves.push(position_converter(target_row + go_back_constant_row, target_column + go_back_constant_column)); //when there is a piece on the way of bishop it cannot go further. If we do not have this boolean variable the loop might continue to go into this if-block although bishop cannot jumpon pieces.
        }

        check_variable = true;
    }

        None => {
            possible_moves.push(position_converter(target_row, target_column))
        }
    }
}

fn is_valid_move_king(gameboard: &[[Option<Piece>; 8]; 8], row: i8, column:i8, main_chosen_piece: &Piece, mut kings_maximal_range: &Vec<String>, mut new_gameboard: &[[Option<Piece>; 8]; 8]) {


    let current_targeted_piece: &Option<Piece> = &gameboard[row as usize][column as usize];

    match current_targeted_piece {

        Some(piece) => {if piece.color != main_chosen_piece.color {
            kings_maximal_range.push(position_converter(row, column));
            new_gameboard[row as usize][column as usize].color = main_chosen_piece.color;
        }}

        None => {
            kings_maximal_range.push(position_converter(row, column))
        }
    }
}
fn get_maximal_range(gameboard: &[[Option<Piece>; 8]; 8], position: String, current_game: &Game) -> Option<Vec<String>> {
    let possible_moves: Vec<String> = Vec::new();
    let [row, column] = reverse_position_converter(position);
    let which_piece: &Option<Piece> = &gameboard[row as usize][column as usize];

    if let Some(chosen_piece) = which_piece {
        if chosen_piece.what_type == "Pawn" {
            if chosen_piece.color == "Black" && row < 7 {
                let piece_at_front: &Option<Piece> = &gameboard[row + 1][column];

                match piece_at_front {
                    Some(_) => (),
                    None => possible_moves.push(position_converter(row + 1, column)),
                }

                if column < 7 {
                    is_valid_move_pawn(&gameboard, row + 1, column + 1, chosen_piece, possible_moves);
                }

                if column > 0 {
                    is_valid_move_pawn(&gameboard, row + 1, column - 1, chosen_piece, possible_moves);
                }

                if row == 1 {
                    let piece_at_double_front: &Option<Piece> = &gameboard[row + 2][column];
                    match piece_at_double_front {
                        Some(_) => (),
                        None => possible_moves.push(position_converter(row + 2, column)),
                    }
                }

                if row == 4 {
                    is_valid_en_passant(&gameboard, row, column, chosen_piece, &mut possible_moves);
                }
            } else if chosen_piece.color == "White" && row > 0 {
                let piece_at_front: &Option<Piece> = &gameboard[row - 1][column];

                match piece_at_front {
                    Some(_) => (),
                    None => possible_moves.push(position_converter(row - 1, column)),
                }

                if column > 0 {
                    is_valid_move_pawn(&gameboard, row - 1, column + 1, chosen_piece, possible_moves);
                }

                if column > 0 {
                    is_valid_move_pawn(&gameboard, row - 1, column - 1, chosen_piece, possible_moves);
                }

                if row == 6 {
                    let piece_at_doube_front: &Option<Piece> = &gameboard[row - 2][column];
                    match piece_at_doube_front {
                        Some(_) => (),
                        None => possible_moves.push(position_converter(row - 2, column)),
                    }
                }

                if row == 3 {
                    is_valid_en_passant(&gameboard, row, column, chosen_piece, &mut possible_moves);
                }
            }
        } else if chosen_piece.what_type == "Knight" {
            if column > 0 {
                if row >= 2 {
                    is_valid_move_knight(&gameboard, row - 2, column - 1, chosen_piece, possible_moves);
                }

                if row <= 5 {
                    is_valid_move_knight(&gameboard, row + 2, column - 1, chosen_piece, possible_moves);
                }
            }

            if column < 7 {
                if row >= 2 {
                    is_valid_move_knight(&gameboard, row - 2, column + 1, chosen_piece, possible_moves);
                }

                if row <= 5 {
                    is_valid_move_knight(&gameboard, row + 2, column + 1, chosen_piece, possible_moves);
                }
            }

            if column <= 5 {
                if row > 1 {
                    is_valid_move_knight(&gameboard, row - 2, column + 1, chosen_piece, possible_moves);
                }

                if row < 6 {
                    is_valid_move_knight(&gameboard, row - 1, column + 2, chosen_piece, possible_moves);
                }
            }

            if column >= 2 {
                if row > 1 {
                    is_valid_move_knight(&gameboard, row - 1, column - 2, chosen_piece, possible_moves);
                }

                if row < 6 {
                    is_valid_move_knight(&gameboard, row + 1, column - 2, chosen_piece, possible_moves);
                }
            }
        } else if chosen_piece.what_type == "Bishop" || chosen_piece.what_type == "Rook" || chosen_piece.what_type == "Queen" {
            if chosen_piece.what_type == "Bishop" || chosen_piece.what_type == "Queen" {
                let mut is_right_up_done = false;
                let mut is_left_up_done = false;
                let mut is_left_down_done = false;
                let mut is_right_down_done = false;

                for i in 1..8 {
                    if row + i <= 7 && column + i <= 7 && !is_right_up_done {
                        is_valid_move_bishop(&gameboard, row, column, chosen_piece, &mut possible_moves, &mut is_right_up_done, true, true, &i);
                    }

                    if row - i >= 0 && column - i >= 0 && !is_left_down_done {
                        is_valid_move_bishop(&gameboard, row, column, chosen_piece, &mut possible_moves, &mut is_left_down_done, false, false, &i);
                    }

                    if row - i >= 0 && column + i <= 7 && !is_left_up_done {
                        is_valid_move_bishop(&gameboard, row, column, chosen_piece, &mut possible_moves, &mut is_left_up_done, false, true, &i);

                        let current_targeted_piece: &Option<Piece> = &gameboard[row - i][column + i];
                    }

                    if row + i <= 7 && column - i >= 0 && !is_right_down_done {
                        is_valid_move_bishop(&gameboard, row, column, chosen_piece, &mut possible_moves, &mut is_right_down_done, true, false, &i);
                    }
                }
            }

            if chosen_piece.what_type == "Rook" || chosen_piece.what_type == "Queen" {
                let mut is_right_done: bool = false;
                let mut is_left_done: bool = false;
                let mut is_up_done: bool = false;
                let mut is_down_done: bool = false;

                for i in 1..8 {
                    if column + i <= 7 && !is_right_done {
                        is_valid_move_rook(&gameboard, row, column, chosen_piece, &mut possible_moves, &mut is_right_done, "column", true, &i);
                    }

                    if column - i >= 0 && !is_left_done {
                        is_valid_move_rook(&gameboard, row, column, chosen_piece, &mut possible_moves, &mut is_left_done, "column", false, &i);
                    }

                    if row + i <= 7 && !is_down_done {
                        is_valid_move_rook(&gameboard, row, column, chosen_piece, &mut possible_moves, &mut is_down_done, "row", true, &i);
                    }

                    if row - i >= 0 && !is_up_done {
                        is_valid_move_rook(&gameboard, row, column, chosen_piece, &mut possible_moves, &mut is_up_done, "row", false, &i);
                    }
                }
            }
        }

        if chosen_piece.what_type == "King" {
            let mut maximal_range_of_king: Vec<String> = Vec::new();
            let mut new_gameboard_to_be_scanned = &gameboard.clone();

            if !chosen_piece.has_moved {
                let rook1: Option<Piece> = &gameboard[row][column + 3];
                let rook2: Option<Piece> = &gameboard[row][column - 4];

                if chosen_piece.color == "White" {
                    match rook1 {
                        Some(piece) => {
                            if piece.what_type == "Rook" {
                                if !piece.has_moved {
                                    if is_empty(&gameboard, "F1") && is_empty(&self, "G1") {
                                        maximal_range_of_king.push("G1");
                                    }
                                }
                            }
                        }
                        None => (),
                    }

                    match rook2 {
                        Some(piece) => {
                            if piece.what_type == "Rook" {
                                if !piece.has_moved {
                                    if is_empty(&gameboard, "D1") && is_empty(&gameboard, "C1") && is_empty(&gameboard, "B1") {
                                        maximal_range_of_king.push("C1");
                                    }
                                }
                            }
                        }
                        None => (),
                    }
                } else if chosen_piece.color == "Black" {
                    match rook1 {
                        Some(piece) => {
                            if piece.what_type == "Rook" {
                                if !piece.has_moved {
                                    if is_empty(&gameboard, "F8") && is_empty(&gameboard, "G8") {
                                        maximal_range_of_king.push("G8");
                                    }
                                }
                            }
                        }
                        None => (),
                    }

                    match rook2 {
                        Some(piece) => {
                            if piece.what_type == "Rook" {
                                if !piece.has_moved {
                                    if is_empty(&gameboard, "D8") && is_empty(&gameboard, "C8") && is_empty(&gameboard, "B8") {
                                        maximal_range_of_king.push("C8");
                                    }
                                }
                            }
                        }
                        None => (),
                    }
                }
            }

            if row > 0 {
                is_valid_move_king(&gameboard, row - 1, column, chosen_piece, maximal_range_of_king, new_gameboard_to_be_scanned);

                if column > 0 {
                    is_valid_move_king(&gameboard, row - 1, column - 1, chosen_piece, maximal_range_of_king, new_gameboard_to_be_scanned);
                }

                if column < 7 {
                    is_valid_move_king(&gameboard, row - 1, column + 1, chosen_piece, maximal_range_of_king, new_gameboard_to_be_scanned);
                }
            }

            if row < 7 {
                is_valid_move_king(&gameboard, row + 1, column, chosen_piece, maximal_range_of_king, new_gameboard_to_be_scanned);

                if column > 0 {
                    is_valid_move_king(&gameboard, row + 1, column - 1, chosen_piece, maximal_range_of_king, new_gameboard_to_be_scanned);
                }

                if column < 7 {
                    is_valid_move_king(&gameboard, row + 1, column + 1, chosen_piece, maximal_range_of_king, new_gameboard_to_be_scanned);
                }
            }

            if column > 0 {
                is_valid_move_king(&gameboard, row, column - 1, chosen_piece, maximal_range_of_king, new_gameboard_to_be_scanned);
            }

            if column < 7 {
                is_valid_move_king(&gameboard, row, column + 1, chosen_piece, maximal_range_of_king, new_gameboard_to_be_scanned);
            }

            if chosen_piece.color == "White" {

                let maximal_range_of_opposite_king = get_opposite_kings_range(&gameboard, &current_game.black_king_position);

            }
            else {

                let maximal_range_of_opposite_king = get_opposite_kings_range(&gameboard, &current_game);
            }

            for moves in maximal_range_of_opposite_king {
                if maximal_range_of_king.contains(moves) {
                    if let Some(index) = maximal_range_of_king.iter().position(|&x| x == moves) {
                        maximal_range_of_king.remove(index);
                    }
                }
            }

            for every_row in 1..8 {
                for every_column in 1..8 {
                    if maximal_range_of_king.contains(new_gameboard_to_be_scanned[every_row as usize][every_column as usize]) {
                        if let Some(index) = maximal_range_of_king.iter().position(|moves| moves == new_gameboard_to_be_scanned[every_row as usize][every_column as usize]) {
                            maximal_range_of_king.remove(index);
                        }
                    }
                }
            }

            for every_move in maximal_range_of_opposite_king {
                possible_moves.push(every_move)
            }

            Some(possible_moves)
        }
    }

    None
}