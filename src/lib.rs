pub mod test;

#[derive(Clone)]
#[derive(Debug)]
struct Piece 
{
        position: String,
        color: String,
        what_type: String,
        has_moved: Option<bool>, // This is for checking if rooks or kings have moved before castling and if en passant is still available to do for pawns
        when_moved_double: Option<u16> // For checking en_passaint we have to know if a pawn moved double steps or not and this variable gives us that option and when the pawn has moved double steps
}

impl Piece 
{
    fn new(position: &str, color: &str, what_type: &str, has_moved: Option<bool>, when_moved_double: Option<u16>) -> Self 
    {
        Piece { position: position.to_string(), color: color.to_string(), what_type: what_type.to_string(), has_moved: has_moved, when_moved_double: when_moved_double}
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
#[derive(Debug)]
enum GameState 
{
    InProgress,
    Check,
    GameOver,
    Checkmate,
    DeadPosition,
}

#[derive(Debug)]
struct Game 
{
    gameboard: [[Option<Piece>; 8]; 8],
    current_state: GameState,
    white_king_position: String, // The gamestate is almost completely determined by if the kings are target of another opposite piece or not. In order to increase effectivity we can store the positions of the kings so that we do not have to go through the whole gameboard once again
    black_king_position: String,
    whose_turn: String,
    turn_counter: u16 
}


impl Game 
{
    pub fn set_promotion(&mut self, piece: String) -> () {

        let current_gameboard = &mut self.gameboard;

        for column in 0..8 {
            
            if let Some(pawn_in_first_row) = current_gameboard[0][column].as_mut() {

                if pawn_in_first_row.what_type == "Pawn".to_string() {

                    pawn_in_first_row.what_type = match piece.as_str(){
                        "Rook" => "Rook".to_string(),
                        "Bishop" => "Bishop".to_string(),
                        "Knight" => "Knight".to_string(),
                        "Queen" => "Queen".to_string(),
                        &_ => todo!()
                    }  
                }


            }

            if let Some(pawn_in_last_row) = current_gameboard[7][column].as_mut() {

                if pawn_in_last_row.what_type == "Pawn".to_string() {

                    pawn_in_last_row.what_type = match piece.as_str(){
                        "Rook" => "Rook".to_string(),
                        "Bishop" => "Bishop".to_string(),
                        "Knight" => "Knight".to_string(),
                        "Queen" => "Queen".to_string(),
                        &_ => todo!()
                    } 
                }

            }

        }

    }

    pub fn ask_for_promotion(&self, piece: String) -> bool {
        let current_gameboard = &self.gameboard;

        for column in 0..8 {
            
            if let Some(pawn_in_first_row) = &current_gameboard[0][column] {

                if pawn_in_first_row.what_type == "Pawn".to_string() {

                    return true;
                }


            }

            if let Some(pawn_in_last_row) = &current_gameboard[7][column] {

                if pawn_in_last_row.what_type == "Pawn".to_string() {

                    return true;
                }
            }

        }

        return false;
    }
    
    pub fn get_game_state(&self) -> GameState {
        let current_gamestate = &self.current_state;
        *current_gamestate
    }

    pub fn get_current_turn(&self) -> u16 {
        self.turn_counter
    }

    pub fn get_whose_turn(&self) -> String {

        self.whose_turn.clone()
    }    
    pub fn make_move(&mut self, from: String, to: String) -> Option<GameState> {
        if matches!(self.current_state, GameState::Check | GameState::InProgress ) { // if it is a check the player still has the opportunity to save their king
            
            let mut is_check_white: bool = false; // this boolean variable is for checking if the kings position is under threat of an enemy piece. If both the position of the king is under threat and the king has nowhere to go it is a checkmate, otherwise if kings position is not under threat it means that it is a stalemate
            let mut is_check_black: bool = false; // the reason we specify colors of the check variables is to not let the player do another move that does not save the king from the threat
            let valid_moves = self.get_possible_moves(&from); // we shall get the valid moves of the wished piece so that we know where it is able to go
            let [row, column] = reverse_position_converter(&from); 
            let [target_row, target_column] = reverse_position_converter(&to);
            let which_piece = self.gameboard[row as usize][column as usize].clone();
            let mut is_there_any_other_move = false;
            let mut white_king_moves: Vec<String> = self.get_possible_moves(&self.white_king_position).unwrap(); // These vectors will be used to check if other opposite pieces block all the way the kings can go to and hence will help us determining if the game is a checkmate, stalemate or something else
            let mut black_king_moves: Vec<String> = self.get_possible_moves(&self.black_king_position).unwrap();

            if let Some(mut chosen_piece) = which_piece {

                if let Some(valid_moves) = valid_moves {
                    
                    if valid_moves.contains(&to) && self.whose_turn == chosen_piece.color  { // if the user has really chosen a piece and the piece is able to move to that chosen position it is valid

                        chosen_piece.position = to.clone();
                        self.gameboard[target_row as usize][target_column as usize] = Some(chosen_piece.clone());
                        self.gameboard[row as usize][column as usize] = None;
                        let will_a_pawn_be_removed: Option<[i8; 2]> = is_move_en_passant(&self.gameboard, &chosen_piece, &to, &self.turn_counter);
                    
                        
                        if chosen_piece.what_type == "Pawn".to_string() {

                            if let Some(pawn_to_be_removed) = will_a_pawn_be_removed {
                                let [pawns_row, pawns_column] = pawn_to_be_removed;
                                self.gameboard[pawns_row as usize][pawns_column as usize] = None;
                            }

                            if ((row as i8) - (target_row as i8)).abs() == 2 {

                                chosen_piece.when_moved_double = Some(self.turn_counter);
                            }

                        }
                        
                        if chosen_piece.what_type == "King".to_string() {
                            if self.whose_turn == "White".to_string() {

                                self.white_king_position = to.clone();
                                if (from == "E1".to_string() && to == "G1".to_string()) {

                                    self.gameboard[7][7].as_mut().unwrap().position = "F1".to_string();
                                    self.gameboard[7][7].as_mut().unwrap().has_moved = Some(true);
                                    self.gameboard[7][5] = Some(self.gameboard[7][7].as_mut().unwrap().clone());
                                    self.gameboard[7][7] = None;

                                } 
                                if (from == "E1".to_string() && to == "C1".to_string()) {
                                    self.gameboard[7][0].as_mut().unwrap().position = "D1".to_string();
                                    self.gameboard[7][0].as_mut().unwrap().has_moved = Some(true);
                                    self.gameboard[7][3] = Some(self.gameboard[7][0].as_mut().unwrap().clone());
                                    self.gameboard[7][0] = None;

                                }

                            }

                            else {

                                self.black_king_position = to.clone();
                               if (from == "E8".to_string() && to == "G8".to_string()) {

                                    self.gameboard[0][7].as_mut().unwrap().position = "F8".to_string();
                                    self.gameboard[0][7].as_mut().unwrap().has_moved = Some(true);
                                    self.gameboard[0][5] = Some(self.gameboard[0][7].as_mut().unwrap().clone());
                                    self.gameboard[0][7] = None;

                                } 
                                if (from == "E8".to_string() && to == "C8".to_string()) {
                                    self.gameboard[0][0].as_mut().unwrap().position = "D8".to_string();
                                    self.gameboard[0][0].as_mut().unwrap().has_moved = Some(true);
                                    self.gameboard[0][3] = Some(self.gameboard[0][0].as_mut().unwrap().clone());
                                    self.gameboard[0][0] = None;

                                }
                            }
                        }

                        if chosen_piece.what_type == "King".to_string() || chosen_piece.what_type == "Rook" {

                            chosen_piece.has_moved = Some(true);
                        }

                        white_king_moves = self.get_possible_moves(&self.white_king_position).unwrap(); // These vectors will be used to check if other opposite pieces block all the way the kings can go to and hence will help us determining if the game is a checkmate, stalemate or something else
                        black_king_moves = self.get_possible_moves(&self.black_king_position).unwrap();
                        
                        for every_row in 0..8 {

                            for every_column in 0..8 {

                                let chosen_piece = self.gameboard[every_row as usize][every_column as usize].clone();
                                if let Some(ref chosen_piece) = chosen_piece {

                                    if chosen_piece.color == "Black".to_string() {

                                        let possible_moves_of_the_enemy = get_maximal_range(&self.gameboard, &position_converter(every_row, every_column), &self);
                                        if possible_moves_of_the_enemy.unwrap().contains(&self.white_king_position) {

                                            is_check_white = true;
                                        }

                                    else if chosen_piece.color == "White".to_string() {

                                        let possible_moves_of_the_enemy = get_maximal_range(&self.gameboard, &position_converter(every_row, every_column), &self);

                                        if possible_moves_of_the_enemy.unwrap().contains(&self.black_king_position) {

                                            is_check_black = true;
                                        }
                                            
                                        }

                                    }

                                    if self.whose_turn !=  chosen_piece.clone().color  && chosen_piece.what_type != "King".to_string() && !is_there_any_other_move {
                                        let possible_friendly_moves = self.get_possible_moves(&position_converter(every_row, every_column)).unwrap();
                                        if possible_friendly_moves.len() > 0 {
                                            is_there_any_other_move = true;
                                        }
                                    }


                                    if self.whose_turn== "White".to_string() && is_check_black { // if we found that it is check already we can break

                                        break;
                                    }

                                    else if self.whose_turn == "Black".to_string() && is_check_white {

                                        break;
                                    }


                                }


                            }
                        }



                    }

                    if self.whose_turn == "White".to_string() {


                        if is_check_black && black_king_moves.len() == 0 {

                            self.current_state = GameState::Checkmate;
                        }

                        else if is_check_black && black_king_moves.len() > 0 {

                            self.current_state = GameState::Check;
                            self.whose_turn = "Black".to_string();
                            self.turn_counter += 1;
                        }

                        else if !is_check_black && black_king_moves.len() == 0 && !is_there_any_other_move {

                            self.current_state = GameState::DeadPosition;
                        }

                        else {

                            self.current_state = GameState::InProgress;
                            self.whose_turn = "Black".to_string();
                            self.turn_counter += 1;
                        }


                        return Some(self.current_state);
                    }

                    if self.whose_turn == "Black".to_string() {

                        if is_check_white && white_king_moves.len() == 0 {

                            self.current_state = GameState::Checkmate;
                        }

                        else if is_check_white && white_king_moves.len() > 0 {

                            self.current_state = GameState::Check;
                            self.whose_turn = "White".to_string();
                            self.turn_counter += 1;
                        }

                        else if !is_check_white && white_king_moves.len() == 0 && !is_there_any_other_move {

                            self.current_state = GameState::DeadPosition;
                        }

                        else {

                            self.current_state = GameState::InProgress;
                            self.whose_turn = "White".to_string();
                            self.turn_counter += 1;
                        }

                        return Some(self.current_state);
                    }

                    else {
                        return None;
                    }
                    
                }

                else {
                    return None;
                }
            }

            else {
                return None;
            }
        }

        else {
            return None;
        }
    }


    

    pub fn get_possible_moves(&self, position: &String) -> Option<Vec<String>>{

        let possible_maximum_range: Option<Vec<String>> = get_maximal_range(&self.gameboard, position, &self);


        match possible_maximum_range {

            Some(maximal_ranges) => {

                let mut actual_range = Vec::new();


                for moves in &maximal_ranges {

                    let mut new_gameboard_to_be_scanned = self.gameboard.clone();
                    let [row, column] = reverse_position_converter(position);
                    let old_chosen_piece: Option<Piece> = self.gameboard[row as usize][column as usize].clone();
                    let mut false_move_detected = false;

                    if let Some(old_chosen_piece) = old_chosen_piece {

                        let [target_row, target_column] = reverse_position_converter(&moves);
                        let mut chosen_piece = old_chosen_piece.clone();
                        chosen_piece.position = moves.to_string();
                        new_gameboard_to_be_scanned[target_row as usize][target_column as usize] = Some(chosen_piece.clone());
                        new_gameboard_to_be_scanned[row as usize][column as usize] = None;

                        if let Some(ref mut piece) = new_gameboard_to_be_scanned[target_row as usize][target_column as usize] {

                            piece.position = moves.to_string();
                        }

                        if chosen_piece.what_type == "Pawn".to_string() {

                            let will_a_pawn_be_removed = is_move_en_passant(&self.gameboard, &chosen_piece, &moves, &self.turn_counter);

                            match will_a_pawn_be_removed {

                                Some(pawn_position) => {

                                    let [pawns_row, pawns_column] = pawn_position;
                                    new_gameboard_to_be_scanned[pawns_row as usize][pawns_column as usize] = None
                                }

                                None => ()
                            }
                        

                        }

                        if chosen_piece.what_type == "King".to_string() {

                            return Some(maximal_ranges);
                        }

                        for rows in 0..8 {

                            for columns in 0..8 {

                                let targeted_piece = new_gameboard_to_be_scanned[rows as usize][columns as usize].clone();

                                if let Some(targeted_piece) = targeted_piece {

                                    if targeted_piece.color != self.whose_turn {

                                        let enemys_move_list = get_maximal_range(&new_gameboard_to_be_scanned, &position_converter(rows as i8, columns as i8), &self);
                        
                                        
                                        if let Some(enemys_move_list) = enemys_move_list {


                                            if self.whose_turn == "White".to_string() {

                                                if enemys_move_list.contains(&self.white_king_position) {

                                                    false_move_detected = true;
                                                    break;
                                                }
                                            }
                                        
                                            else {

                                                if enemys_move_list.contains(&self.black_king_position) {
                                                    
                                                    false_move_detected = true;

                                                    break;
                                                }
                                            }

                                        }

                                    }
                                }
                            }

                            if false_move_detected {

                                break;
                            }
                        }
                        

                    }

                    if !false_move_detected {
                        actual_range.push(moves.to_string());
                    }
                    
                }
                

                return Some(actual_range);
            }

            None => return None
        }

        return None;
    }        
        
    
    pub fn new() -> Game {

        let mut chessboard: [[Option<Piece>; 8]; 8] = std::array::from_fn(|_| std::array::from_fn(|_| None));

        for column in 0..8
        {

            if column == 0 || column == 7
            {
            
                chessboard[0][column] = Some(Piece::new(&position_converter(0 as i8, column as i8), "Black", "Rook", Some(false), None)); 
                chessboard[1][column] = Some(Piece::new(&position_converter(1 as i8, column as i8), "Black", "Pawn", None, Some(0))); // At start, in every column there is two pawns and two different pieces on both sides which respectively are black and white. That's why we have the numbers like 0, 1, 6 ,7 (one special piece and pawn at one end and vice versa at the other end of the table)
                chessboard[6][column] = Some(Piece::new(&position_converter(6 as i8, column as i8), "White", "Pawn", None, Some(0)));
                chessboard[7][column] = Some(Piece::new(&position_converter(7 as i8, column as i8), "White", "Rook", Some(false),None)); //On contrary to rooks, pawns have the variable 'has_mode' on None by default. This variable is just for checking if rooks or kings have moved before castling

            }

            else if column == 1 || column == 6
            {
            
                chessboard[0][column] = Some(Piece::new(&position_converter(0 as i8, column as i8), "Black", "Knight", None, None)); 
                chessboard[1][column] = Some(Piece::new(&position_converter(1 as i8, column as i8), "Black", "Pawn", None, Some(0))); 
                chessboard[6][column] = Some(Piece::new(&position_converter(6 as i8, column as i8), "White", "Pawn", None, Some(0)));
                chessboard[7][column] = Some(Piece::new(&position_converter(7 as i8, column as i8), "White", "Knight", None, None)); 
            }

            else if column == 2 || column == 5
            {
            
                chessboard[0][column] = Some(Piece::new(&position_converter(0 as i8, column as i8), "Black", "Bishop", None, None)); 
                chessboard[1][column] = Some(Piece::new(&position_converter(1 as i8, column as i8), "Black", "Pawn", None, Some(0))); 
                chessboard[6][column] = Some(Piece::new(&position_converter(6 as i8, column as i8), "White", "Pawn", None, Some(0)));
                chessboard[7][column] = Some(Piece::new(&position_converter(7 as i8, column as i8), "White", "Bishop", None,None)); 
            }

            else if column == 3
            {
            
                chessboard[0][column] = Some(Piece::new(&position_converter(0 as i8, column as i8), "Black", "Queen", None, None)); 
                chessboard[1][column] = Some(Piece::new(&position_converter(1 as i8, column as i8), "Black", "Pawn", Some(false), Some(0))); 
                chessboard[6][column] = Some(Piece::new(&position_converter(6 as i8, column as i8), "White", "Pawn", Some(false), Some(0)));
                chessboard[7][column] = Some(Piece::new(&position_converter(7 as i8, column as i8), "White", "Queen", None, None)); 
            }

            else if column == 4
            {
            
                chessboard[0][column] = Some(Piece::new(&position_converter(0 as i8, column as i8), "Black", "King", Some(false), None)); 
                chessboard[1][column] = Some(Piece::new(&position_converter(1 as i8, column as i8), "Black", "Pawn", Some(false), Some(0))); 
                chessboard[6][column] = Some(Piece::new(&position_converter(6 as i8, column as i8), "White", "Pawn", Some(false), Some(0)));
                chessboard[7][column] = Some(Piece::new(&position_converter(7 as i8, column as i8), "White", "King", Some(false), None)); 
            }
        }
        
        let new_gameboard: Game = Game { gameboard: chessboard, current_state: GameState::InProgress, white_king_position: "E1".to_string(), black_king_position: "E8".to_string(), whose_turn: "White".to_string(), turn_counter: 1};
        new_gameboard
                    
    }

}

// -----------------------------------------------------------------------------
// *************************** HELPER FUNCTIONS ********************************
// -----------------------------------------------------------------------------

fn position_converter(x: i8, y: i8) -> String 
{

    let row = (8 as i8 - x).to_string(); 
    
    // 65 is A's ASCII chart number, in Rust chars can also be modified as if they are numbers and there is a special chart (ASCII chart) for that
    let column: char = char::from_u32((y + 65) as u32).unwrap();
    

    let position: String = format!("{}{}", column, row);

    position
}

fn reverse_position_converter(position: &String) -> [i8; 2]
{   
    let chars: Vec<char> = position.chars().collect();
    let column = (chars[0] as u8 - b'A') as i8; 
    let row = chars[1].to_digit(10).unwrap() as i8 - 1;
    let coordinates: [i8; 2] = [7 - row, column];
    coordinates
}


fn get_opposite_kings_range(gameboard: &[[Option<Piece>; 8]; 8], opposite_king_position: &String) -> Vec<String>  // The reason of this function's existence is that when two kings can never be side by side. They shall at least have  a distance of one square and as the get_possible_moves is having recursion it would throw out an error if it were to come to the king. 
{

    let mut maximal_range_of_opposite_king: Vec<String> = Vec::new();
    let [row, column] = reverse_position_converter(opposite_king_position);
    let chosen_piece = &gameboard[row as usize][column as usize];

    
    if let Some(chosen_piece) = chosen_piece {
        if row > 0
        {   
            is_valid_move_opposite_king(gameboard, row - 1, column, chosen_piece, &mut maximal_range_of_opposite_king);
            if column > 0
            {
                is_valid_move_opposite_king(gameboard, row - 1, column - 1, chosen_piece, &mut maximal_range_of_opposite_king);
            }

            if column < 7
            {
                is_valid_move_opposite_king(gameboard, row - 1, column + 1, chosen_piece, &mut maximal_range_of_opposite_king);

            }
        }
        if row < 7
        {   
            is_valid_move_opposite_king(gameboard, row + 1, column, chosen_piece, &mut maximal_range_of_opposite_king);
             
            if column > 0
            {
                is_valid_move_opposite_king(gameboard, row + 1, column - 1, chosen_piece, &mut maximal_range_of_opposite_king); 
            }

            if column < 7
            {
                is_valid_move_opposite_king(gameboard, row + 1, column + 1, chosen_piece, &mut maximal_range_of_opposite_king);
            }
        }
        if column > 0 {

            is_valid_move_opposite_king(gameboard, row , column - 1, chosen_piece, &mut maximal_range_of_opposite_king); 
        }
        
        if column < 7
        {
            is_valid_move_opposite_king(gameboard, row, column + 1, chosen_piece, &mut maximal_range_of_opposite_king);
        }
    }   
        return maximal_range_of_opposite_king;
}





fn is_empty(gameboard: &[[Option<Piece>; 8]; 8], position: &String) -> bool
{   
    let [row, column] = reverse_position_converter(&position);
    let current_targeted_piece = &gameboard[row as usize][column as usize];

    match current_targeted_piece {
        Some(_) => {
            false
        },
        None => {
            true
        }
    }
}


fn is_valid_move_pawn(gameboard: &[[Option<Piece>; 8]; 8], row: i8, column:i8, main_chosen_piece: &Piece, possible_moves: &mut Vec<String>)
{   
    let current_targeted_piece = &gameboard[row as usize][column as usize];

    match current_targeted_piece {

        Some(piece) => {if main_chosen_piece.color != piece.color {
            possible_moves.push(position_converter(row, column));
        }},

        None => {
            ()
        }
    }

}

fn is_valid_en_passant(gameboard: &[[Option<Piece>; 8]; 8], row: i8, column:i8, main_chosen_piece: &Piece, possible_moves: &mut Vec<String>, which_turn: &u16) {

    let mut go_forward_constant: i8 = 0;

    if main_chosen_piece.color == "Black" {

        go_forward_constant = 1; // if a black pawn will go forward it means that it will be placed in an array that is located closer to the end and vice versa for the white

    }

    else {
        
        go_forward_constant = -1;
    }

    if column < 7 {
        let piece_at_right: &Option<Piece> = &gameboard[row as usize][(column + 1) as usize];

        match piece_at_right {
            Some(piece) => {if piece.color != main_chosen_piece.color && which_turn - piece.when_moved_double.unwrap() == 1 && piece.what_type == "Pawn".to_string() {
                let piece_on_diagonal: &Option<Piece> = &gameboard[(row + go_forward_constant) as usize][(column + 1) as usize];
            
                match &piece_on_diagonal {

                    Some(_) => (), // if let say there is a white piece at the square which is located diagonally, the coordinate of that piece will be put as there is already a check for it in the pawn part of the get_possible_moves
                
                    None => possible_moves.push(position_converter(row + go_forward_constant, column + 1))
                }
            }}

            None => ()

        }
    }

    if column > 0 {
        let piece_at_left: &Option<Piece> = &gameboard[row as usize][(column - 1) as usize];

        match piece_at_left {

            Some(piece) => {if piece.color != main_chosen_piece.color && which_turn - piece.when_moved_double.unwrap() == 1 && piece.what_type == "Pawn".to_string() {
                let piece_on_diagonal: &Option<Piece> = &gameboard[(row + go_forward_constant) as usize][(column - 1) as usize];
            
                match piece_on_diagonal {

                    Some(_) => (), 
                
                    None => possible_moves.push(position_converter(row + go_forward_constant, column - 1))
                }
            }}

            None => ()

        }
    }


}

fn is_move_en_passant(gameboard: &[[Option<Piece>; 8]; 8], main_chosen_piece: &Piece, target_position: &String, which_turn: &u16) -> Option<[i8; 2]>  // If the move is an en passant it will return the position of the pawn that is being captured by the other pawn
{
    let [row, column] = reverse_position_converter(&main_chosen_piece.position);
    let mut get_forward_constant = 0;

    if row == 3 && main_chosen_piece.color == "White" {

        get_forward_constant= -1;
    }

    else if row == 4 && main_chosen_piece.color == "Black" {

        get_forward_constant= 1;

    }

    else {

        return None;
    }

    if is_empty(&gameboard, target_position) {

        if position_converter(row + get_forward_constant, column - 1) == *target_position && is_empty(&gameboard, &position_converter(row, column - 1)) {

            let current_targeted_piece = &gameboard[row as usize][(column - 1) as usize];

            match current_targeted_piece {

                Some(piece) => {if piece.what_type == "Pawn" && piece.color != main_chosen_piece.color {

                    if piece.has_moved.unwrap() && which_turn - piece.when_moved_double.unwrap() == 1 {

                        return Some(reverse_position_converter(&piece.position));
                    }

                    else {
                        return None;
                    }
                }

                else {
                    return None;
                }
            }

                None => return None
            }
        }

        else if &position_converter(row + get_forward_constant, column + 1) == target_position && is_empty(&gameboard, &position_converter(row, column + 1)) {

            let current_targeted_piece = &gameboard[row as usize][(column + 1) as usize];

            match current_targeted_piece {

                Some(piece) => {if piece.what_type == "Pawn" && piece.color != main_chosen_piece.color {
                    if piece.has_moved.unwrap() && (which_turn - piece.when_moved_double.unwrap())== 1 {

                        Some(reverse_position_converter(&piece.position))
                    }

                    else {
                        None
                    }
                }

                else {

                    return None;
                }
                
                }

                None => None
            
            }

        }

        else {

            return None;
        }

    }

    else {

        return None;
    }
    
}

fn is_valid_move_knight(gameboard: &[[Option<Piece>; 8]; 8], row: &i8, column: &i8, main_chosen_piece: &Piece, possible_moves: &mut Vec<String>)
{   
    let current_targeted_piece = &gameboard[*row as usize][*column as usize];

    match current_targeted_piece {
        Some(piece) => {if main_chosen_piece.color != piece.color {
            possible_moves.push(position_converter(*row, *column));
        }},
        None => {
            possible_moves.push(position_converter(*row, *column))
        }
    }

}

fn is_valid_move_bishop(gameboard: &[[Option<Piece>; 8]; 8], row: i8, column:i8, main_chosen_piece: &Piece, possible_moves: &mut Vec<String>, mut check_variable: &mut bool, row_constant_positive: bool, column_constant_positive: bool, loop_variable: &i8) {

        let mut target_row = 0;
        let mut target_column = 0;
        let mut go_back_constant_row = 0;
        let mut go_back_constant_column = 0;

    
    if row_constant_positive && column_constant_positive {
        target_row = row + loop_variable;
        target_column = column + loop_variable;
        go_back_constant_row = -1;
        go_back_constant_column = -1;
    }

    else if !row_constant_positive && column_constant_positive {
        target_row = row - loop_variable;
        target_column = column + loop_variable;
        go_back_constant_row = 1;
        go_back_constant_column = -1;
    }

    else if row_constant_positive && !column_constant_positive {
        target_row = row + loop_variable;
        target_column = column - loop_variable;
        go_back_constant_row = -1;
        go_back_constant_column = 1;
    }

    else {
        target_row = row - loop_variable;
        target_column = column - loop_variable;
        go_back_constant_row = 1;
        go_back_constant_column = 1;
    }
    
    let current_targeted_piece: &Option<Piece> = &gameboard[target_row as usize][target_column as usize];

    match current_targeted_piece
    {
        Some(piece) => {if piece.color != main_chosen_piece.color
        { possible_moves.push(position_converter(target_row, target_column))}

        *check_variable = true;
    }

        None => {
            possible_moves.push(position_converter(target_row, target_column))
        }
    }
}

fn is_valid_move_rook(gameboard: &[[Option<Piece>; 8]; 8], row: i8, column:i8, main_chosen_piece: &Piece, possible_moves: &mut Vec<String>, check_variable: &mut bool, row_or_column: &str, constant_positive: bool, loop_variable: &i8) {
    let mut target_row = 0;
    let mut target_column = 0;
    let mut go_back_constant_row = 0;
    let mut go_back_constant_column = 0;

    if row_or_column == "row"
    {
        if constant_positive
        {
            target_row = row + loop_variable;
            target_column = column;
            go_back_constant_row = -1;
            go_back_constant_column = 0;

        }
        
        else 
        {
            target_row = row - loop_variable;
            target_column = column;
            go_back_constant_row = 1;
            go_back_constant_column = 0; 
        }
    }

    else if row_or_column == "column"
    {
        if constant_positive
        {
            target_row = row;
            target_column = column + loop_variable;
            go_back_constant_row = 0;
            go_back_constant_column = -1;

        }
        
        else 
        {
            target_row = row;
            target_column = column - loop_variable; 
            go_back_constant_row = 0;
            go_back_constant_column = 1;

        }
    }

    let current_targeted_piece: &Option<Piece> = &gameboard[target_row as usize][target_column as usize];

    match current_targeted_piece
    {
        Some(piece) => {
            if piece.color != main_chosen_piece.color{ 
                possible_moves.push(position_converter(target_row, target_column))
            }

            *check_variable = true;
        }

        None => {
            possible_moves.push(position_converter(target_row, target_column));
           
        }
    }
}
fn is_valid_move_opposite_king(gameboard: &[[Option<Piece>; 8]; 8], row: i8, column: i8, main_chosen_piece: &Piece, kings_maximal_range: &mut Vec<String>) {


    let current_targeted_piece: &Option<Piece> = &gameboard[row as usize][column as usize];

    match current_targeted_piece {

        Some(piece) => {if piece.color != main_chosen_piece.color {
            kings_maximal_range.push(position_converter(row, column));
        }}

        None => {
            kings_maximal_range.push(position_converter(row, column))
        }
    }
}
fn is_valid_move_king(gameboard: &[[Option<Piece>; 8]; 8], row: i8, column:i8, main_chosen_piece: &Piece, kings_maximal_range: &mut Vec<String>, new_gameboard: &mut [[Option<Piece>; 8]; 8]) {


    let current_targeted_piece: &Option<Piece> = &gameboard[row as usize][column as usize];

    match current_targeted_piece {

        Some(piece) => {if piece.color != main_chosen_piece.color {
            kings_maximal_range.push(position_converter(row, column));

            if let Some(ref mut new_gameboard_piece) = new_gameboard[row as usize][column as usize] {

                new_gameboard_piece.color = main_chosen_piece.color.clone();
            }
        }}

        None => {
            kings_maximal_range.push(position_converter(row, column))
        }
    }
}
fn get_maximal_range(gameboard: &[[Option<Piece>; 8]; 8], position: &String, current_game: &Game) -> Option<Vec<String>> {
    let mut possible_moves: Vec<String> = Vec::new();
    let [row, column] = reverse_position_converter(position);
    let which_piece: &Option<Piece> = &gameboard[row as usize][column as usize];

    if let Some(chosen_piece) = which_piece {
        if chosen_piece.what_type == "Pawn" {
            if chosen_piece.color == "Black" && row < 7 {
                let piece_at_front: &Option<Piece> = &gameboard[(row + 1) as usize][column as usize];

                match piece_at_front {
                    Some(_) => (),
                    None => possible_moves.push(position_converter(row + 1, column)),
                }

                if column < 7 {
                    is_valid_move_pawn(&gameboard, row + 1, column + 1, chosen_piece, &mut possible_moves);
                }

                if column > 0 {
                    is_valid_move_pawn(&gameboard, row + 1, column - 1, chosen_piece, &mut possible_moves);
                }

                if row == 1 {
                    let piece_at_double_front: &Option<Piece> = &gameboard[(row + 2) as usize][column as usize];
                    match piece_at_double_front {
                        Some(_) => (),
                        None => {if is_empty(&gameboard, &position_converter(row + 1, column)) {
                            possible_moves.push(position_converter(row + 2, column))}
                        }
                    }
                }

                if row == 4 {
                    is_valid_en_passant(&gameboard, row, column, chosen_piece, &mut possible_moves, &current_game.turn_counter);
                }
            } else if chosen_piece.color == "White" && row > 0 {
                let piece_at_front: &Option<Piece> = &gameboard[(row - 1) as usize][column as usize];

                match piece_at_front {
                    Some(_) => (),
                    None => possible_moves.push(position_converter(row - 1, column)),
                }

                if column < 7 {
                    is_valid_move_pawn(&gameboard, row - 1, column + 1, chosen_piece, &mut possible_moves);
                }

                if column > 0 {
                    is_valid_move_pawn(&gameboard, row - 1, column - 1, chosen_piece, &mut possible_moves);
                }

                if row == 6 {
                    let piece_at_doube_front: &Option<Piece> = &gameboard[(row - 2) as usize][column as usize];
                    match piece_at_doube_front {
                        Some(_) => (),
                        None => {if is_empty(&gameboard, &position_converter(row - 1, column)) {
                            possible_moves.push(position_converter(row - 2, column))}
                        } 
                    }
                }

                if row == 3 {
                    is_valid_en_passant(&gameboard, row, column, chosen_piece, &mut possible_moves, &current_game.turn_counter);
                }
            }
        } else if chosen_piece.what_type == "Knight" {
            if column > 0 {
                if row >= 2 {
                    is_valid_move_knight(&gameboard, &(row - 2), &(column - 1), chosen_piece, &mut possible_moves);
                }

                if row <= 5 {
                    is_valid_move_knight(&gameboard, &(row + 2), &(column - 1), chosen_piece, &mut possible_moves);
                }
            }

            if column < 7 {
                if row >= 2 {
                    is_valid_move_knight(&gameboard, &(row - 2), &(column + 1), chosen_piece, &mut possible_moves);
                }

                if row <= 5 {
                    is_valid_move_knight(&gameboard, &(row + 2), &(column + 1), chosen_piece, &mut possible_moves);
                }
            }

            if column <= 5 {
                if row >= 1 {
                    is_valid_move_knight(&gameboard, &(row - 1), &(column + 2), chosen_piece, &mut possible_moves);
                }

                if row <= 6 {
                    is_valid_move_knight(&gameboard, &(row + 1), &(column + 2), chosen_piece, &mut possible_moves);
                }
            }

            if column >= 2 {
                if row >= 1 {
                    is_valid_move_knight(&gameboard, &(row - 1), &(column - 2), chosen_piece, &mut possible_moves);
                }

                if row <= 6 {
                    is_valid_move_knight(&gameboard, &(row + 1), &(column - 2), chosen_piece, &mut possible_moves);
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
            let mut new_gameboard_to_be_scanned = gameboard.clone();

            if let Some(has_moved) = chosen_piece.has_moved {
                if !has_moved {

                    let rook1 = gameboard[row as usize][0].clone();
                    let rook2 = gameboard[row as usize][7].clone();

                    if chosen_piece.color == "White" {
                        match rook1 {
                            Some(piece) => {
                                if piece.what_type == "Rook" {
                                    if !piece.has_moved.unwrap() {
                                        if is_empty(&gameboard, &"F1".to_string()) && is_empty(&gameboard, &"G1".to_string()) {
                                            maximal_range_of_king.push("G1".to_string());
                                        }
                                    }
                                }
                            }
                            None => (),
                        }

                        match rook2 {
                            Some(piece) => {
                                if piece.what_type == "Rook" {
                                    if !piece.has_moved.unwrap() {
                                        if is_empty(&gameboard, &"D1".to_string()) && is_empty(&gameboard, &"C1".to_string()) && is_empty(&gameboard, &"B1".to_string()) {
                                            maximal_range_of_king.push("C1".to_string());
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
                                    if !piece.has_moved.unwrap() {
                                        if is_empty(&gameboard, &"F8".to_string()) && is_empty(&gameboard, &"G8".to_string()) {
                                            maximal_range_of_king.push("G8".to_string());
                                        }
                                    }
                                }
                            }
                            None => (),
                        }

                        match rook2 {
                            Some(piece) => {
                                if piece.what_type == "Rook" {
                                    if !piece.has_moved.unwrap() {
                                        if is_empty(&gameboard, &"D8".to_string()) && is_empty(&gameboard, &"C8".to_string()) && is_empty(&gameboard, &"B8".to_string()) {
                                            maximal_range_of_king.push("C8".to_string());
                                        }
                                    }
                                }
                            }
                            None => (),
                        }
                    }

                }

            }
        
            if row > 0 {
                is_valid_move_king(&gameboard, row - 1, column, chosen_piece, &mut maximal_range_of_king, &mut new_gameboard_to_be_scanned);

                if column > 0 {
                    is_valid_move_king(&gameboard, row - 1, column - 1, chosen_piece, &mut maximal_range_of_king, &mut new_gameboard_to_be_scanned);
                }

                if column < 7 {
                    is_valid_move_king(&gameboard, row - 1, column + 1, chosen_piece, &mut maximal_range_of_king, &mut new_gameboard_to_be_scanned);
                }
            }

            if row < 7 {
                is_valid_move_king(&gameboard, row + 1, column, chosen_piece, &mut maximal_range_of_king, &mut new_gameboard_to_be_scanned);

                if column > 0 {
                    is_valid_move_king(&gameboard, row + 1, column - 1, chosen_piece, &mut maximal_range_of_king, &mut new_gameboard_to_be_scanned);
                }

                if column < 7 {
                    is_valid_move_king(&gameboard, row + 1, column + 1, chosen_piece, &mut maximal_range_of_king, &mut new_gameboard_to_be_scanned);
                }
            }

            if column > 0 {
                is_valid_move_king(&gameboard, row, column - 1, chosen_piece, &mut maximal_range_of_king, &mut new_gameboard_to_be_scanned);
            }

            if column < 7 {
                is_valid_move_king(&gameboard, row, column + 1, chosen_piece, &mut maximal_range_of_king, &mut new_gameboard_to_be_scanned);
            }

            if chosen_piece.color == "White" {

                let maximal_range_of_opposite_king = get_opposite_kings_range(&gameboard, &current_game.black_king_position);

                for moves in maximal_range_of_opposite_king {
                    if maximal_range_of_king.contains(&moves) {
                        if let Some(index) = maximal_range_of_king.iter().position(|x| *x == moves) {
                            maximal_range_of_king.remove(index);
                        }
                    }
                }
            }

            else {

                let maximal_range_of_opposite_king = get_opposite_kings_range(&gameboard, &current_game.white_king_position);

                for moves in maximal_range_of_opposite_king {
                    if maximal_range_of_king.contains(&moves) {
                        if let Some(index) = maximal_range_of_king.iter().position(|x| *x == moves) {
                            maximal_range_of_king.remove(index);
                        }
                    }
                }
            }

            for every_row in 1..8 {
                for every_column in 1..8 {

                    if !is_empty(&new_gameboard_to_be_scanned, &position_converter(every_row, every_column)) {

                        let currently_targeted_piece = &new_gameboard_to_be_scanned[every_row as usize][every_column as usize];

                        if let Some(piece) = currently_targeted_piece {
                                   
                            if piece.color != chosen_piece.color && piece.what_type != "King" {

                                let enemy_piece_moves = get_maximal_range(&new_gameboard_to_be_scanned, &position_converter(every_row, every_column), current_game);

                                if let Some(enemy_piece_moves) = enemy_piece_moves {

                                    for every_move in enemy_piece_moves {
                                        if maximal_range_of_king.contains(&every_move) {
                                            if let Some(index) = maximal_range_of_king.iter().position(|moves| *moves == every_move) {
                                                maximal_range_of_king.remove(index);
                                            }
                                        }
                                    }
                            
                                }

                            }
                        }
                    }

                }
            }

            for every_move in maximal_range_of_king {
                possible_moves.push(every_move);
            }
        }
            return Some(possible_moves);
        }


    None

}

