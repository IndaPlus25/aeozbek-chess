use std::iter;

struct Piece 
{
        position: String,
        color: String,
        what_type: String,
        has_moved: Option<bool>

}

impl Piece 
{
    fn new(position: &str, color: &str, what_type: &str, has_moved: Option<bool>) -> Self 
    {

        Piece { position: position.to_string(), color: color.to_string(), what_type: what_type.to_string(), has_moved: has_moved}
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
    current_state: GameState
}

impl Game 
{
    
    pub fn get_possible_moves(&self, position: String) -> Option<Vec<String>>
    {
        let possible_moves: vec = Vec::new();
        let (row, column): (u8, u8) = reverse_position_converter(position);
        let which_piece: &Option<Piece> = &self.gameboard[row][column];

        if let Some(chosen_piece) = which_piece
        {
            if chosen_piece.what_type == "Pawn" 
            {

                if chosen_piece.color == "Black" && row < 7
                {

                    let piece_at_front: &Option<Piece> = &self.gameboard[row + 1][column];

                    match piece_at_front
                    {
                        Some(_) => (),
                        None => possible_moves.push(position_converter(row, column))
                    }

                    if column < 7
                    {
                        let piece_on_right: &Option<Piece> = &self.gameboard[row + 1][column + 1];

                        match piece_on_right 
                        {
                            Some(piece) => {if piece.color == "White" {

                                possible_moves.push(position_converter(row + 1, column + 1));
                            }},
                            None => ()
                        }
                    }
                    
                    if column > 0
                    {
                        let piece_on_left: &Option<Piece> = &self.gameboard[row + 1][column - 1];

                        match piece_on_left
                        {
                            Some(piece) => {if piece.color == "White" {

                                possible_moves-push(position_converter(row + 1, column - 1));
                            }},
                            None => ()
                        }
                    }
                }

                else if (chosen_piece.color == "White" && row > 0)
                {

                    let piece_at_front: &Option<Piece> = &self.gameboard[row - 1][column];

                    match piece_at_front
                    {
                        Some(_) => (),
                        None => possible_moves.push(position_converter(row - 1, column))
                    }

                    if column > 0
                    {
                        let piece_on_right: &Option<Piece> = &self.gameboard[row - 1][column + 1];

                        match piece_on_right 
                        {
                            Some(piece) => {if piece.color == "White" {

                                possible_moves-push(position_converter(row - 1, column + 1));
                            }},
                            None => ()
                        }
                    }
                    
                    if column > 0
                    {
                        let piece_on_left: &Option<Piece> = &self.gameboard[row - 1][column - 1];

                        match piece_on_left
                        {
                            Some(piece) => {if piece.color == "White" {

                                possible_moves-push(position_converter(row - 1, column - 1));
                            }},
                            None => ()
                        }
                    }

                }

            }

            else if chosen_piece.what_type == "Knight"
            {
                if column > 0 
                {
                    
                    if row >= 2 
                    {

                        let targeted_piece_one: &Option<Piece> = &self.gameboard[row - 2][column - 1];

                        match targeted_piece_one
                        {

                            Some(piece ) => {if piece.color != chosen_piece.color
                            {
                                possible_moves.push(position_converter(row - 2, column - 1));
                            }}

                            None => possible_moves.push(position_converter(row - 2, column - 1))
                        }
                    }

                    if row <= 5
                    {

                        let targeted_piece_two: &Option<Piece> = &self.gameboard[row + 2][column - 1];

                        match targeted_piece_two
                        {

                            Some(piece ) => {if piece.color != chosen_piece.color
                            {
                                possible_moves.push(position_converter(row + 2, column - 1));
                            }}

                            None => possible_moves.push(position_converter(row + 2, column - 1))
                        }

                    }

                }

                
                if column < 7
                {
                    
                    if row >= 2 
                    {

                        let targeted_piece_three: &Option<Piece> = &self.gameboard[row - 2][column + 1];

                        match targeted_piece_three
                        {

                            Some(piece ) => {if piece.color != chosen_piece.color
                            {
                                possible_moves.push(position_converter(row - 2, column + 1));
                            }}

                            None => possible_moves.push(position_converter(row - 2, column + 1))
                        }

                    }

                    if row <= 5
                    {

                        let targeted_piece_four: &Option<Piece> = &self.gameboard[row + 2][column + 1];

                        match targeted_piece_four
                        {

                            Some(piece ) => {if piece.color != chosen_piece.color
                            {
                                possible_moves.push(position_converter(row + 2, column + 1));
                            }}

                            None => possible_moves.push(position_converter(row + 2, column + 1))
                        }

                    }

                }

                if column <= 5
                {
                    if row > 1
                    {

                        let targeted_piece_five: &Option<Piece> = &self.gameboard[row - 1][column + 2];

                        match targeted_piece_five
                        {

                            Some(piece ) => {if piece.color != chosen_piece.color
                            {
                                possible_moves.push(position_converter(row - 1, column + 2));
                            }}

                            None => possible_moves.push(position_converter(row - 1, column + 2))
                        }

                    }

                    if row < 6
                    {

                        let targeted_piece_six: &Option<Piece> = &self.gameboard[row + 1][column + 2];

                        match targeted_piece_six
                        {

                            Some(piece ) => {if piece.color != chosen_piece.color
                            {
                                possible_moves.push(position_converter(row - 1, column + 2));
                            }}

                            None => possible_moves.push(position_converter(row - 1, column + 2))
                        }

                    }
                }

                if column >= 2
                {
                    if row > 1
                    {

                        let targeted_piece_seven: &Option<Piece> = &self.gameboard[row - 1][column - 2];

                        match targeted_piece_seven
                        {

                            Some(piece ) => {if piece.color != chosen_piece.color
                            {
                                possible_moves.push(position_converter(row - 1, column - 2));
                            }}

                            None => possible_moves.push(position_converter(row - 1, column - 2))
                        }

                    }

                    if row < 6
                    {

                        let targeted_piece_eight: &Option<Piece> = &self.gameboard[row + 1][column - 2];

                        match targeted_piece_eight
                        {

                            Some(piece ) => {if piece.color != chosen_piece.color
                            {
                                possible_moves.push(position_converter(row + 1, column - 2));
                            }}

                            None => possible_moves.push(position_converter(row + 1, column - 2))
                        }

                    }
                }
                }
            }

            else if chosen_piece.what_type == "Bishop" || chosen_piece.what_type == "Rook" || chosen_piece.what_type == "Queen"
            {

                if chosen_piece.what_type == "Bishop" || chosen_piece.what_type == "Queen"
                {
                    let mut is_right_up_done = false;
                    let mut is_left_up_done = false;
                    let mut is_left_down_done = false;
                    let mut is_right_down_done = false;

                    for i in 1..8
                    {
                        
                        if row + i <= 7 && column + i <= 7 && !is_right_up_done
                        {
                            let current_targeted_piece: &Option<Piece> = &self.gameboard[row + i][column + i];

                            match current_targeted_piece
                            {
                                Some(piece) => {if piece.color != chosen_piece.color
                                { possible_moves.push(position_converter(row + i, column + i))}

                                else 
                                {
                                    possible_moves.push(position_converter(row + i -1, column + i - 1));
                                    
                                }

                                is_right_up_done = true; //when there is a piece on the way of bishop it cannot go further. If we do not have this boolean variable the loop might continue to go into this if-block although bishop cannot jumpon pieces.
                            }

                                None => possible_moves.push(position_converter(row + i, column + i))
                            }
                        }

                        if row - i >= 0 && column - i >= 0 && !is_left_down_done
                        {
                            let current_targeted_piece: &Option<Piece> = &self.gameboard[row - i][column - i];

                            match current_targeted_piece
                            {
                                Some(piece) => {if piece.color != chosen_piece.color
                                { possible_moves.push(position_converter(row - i, column - i))}

                                else 
                                {
                                    possible_moves.push(position_converter(row - i + 1, column - i + 1));
                                }

                                is_left_down_done = true;
                            }

                                None => possible_moves.push(position_converter(row - i, column - i))
                            }
                        }

                        if row - i >= 0 && column + i <= 7 && !is_left_up_done
                        {
                            let current_targeted_piece: &Option<Piece> = &self.gameboard[row - i][column + i];

                            match current_targeted_piece
                            {
                                Some(piece) => {if piece.color != chosen_piece.color
                                { possible_moves.push(position_converter(row - i, column + i))}

                                else 
                                {
                                    possible_moves.push(position_converter(row - i + 1, column - i - 1));
                                }

                                is_left_up_done = true;
                            }

                                None => possible_moves.push(position_converter(row - i, column + i))
                                
                            }
                        }

                        if row + i <= 7 && column - i >= 0 && !is_right_down_done
                        {
                            let current_targeted_piece: &Option<Piece> = &self.gamebıard[row + i][column - i];

                            match current_targeted_piece
                            {
                                Some(piece) => {if piece.color != chosen_piece.color
                                { possible_moves.push(position_converter(row + i, column - i))}

                                else 
                                {
                                    possible_moves.push(position_converter(row + i - 1, column + i + 1));
                                }

                                is_right_down_done = true;
                            }

                                None => possible_moves.push(position_converter(row + i, column - i))
                            }
                    }

                    if chosen_piece.what_type == "Rook" || chosen_piece.what_type == "Queen"
                    {
                        let mut is_right_done: bool = false;
                        let mut is_left_done: bool = false;
                        let mut is_up_done: bool = false;
                        let mut is_down_done: bool = false;
                        for i in 1..8
                        {
                            if column + i <= 7 && !is_right_done
                            {    
                                let current_targeted_piece: &Option<Piece> = &self.gameboard[row][column + i];

                                match current_targeted_piece
                                {
                                    Some(piece) => { if piece.color != chosen_piece.color
                                        {
                                            possible_moves.push(position_converter(row, column + i));
                                        }

                                        else {
                                            possible_moves.push(position_converter(row, column + i - 1));
                                        }

                                        is_right_done = true;
                                    }

                                    None => possible_moves.push(position_converter(row, column + i))
                                }
                            } 

                            if column - i >= 0 && !is_left_done
                            {
                                let current_targeted_piece: &Option<Piece> = &self.gameboard[row][column - i];

                                match current_targeted_piece
                                {
                                    Some(piece) => { if piece.color != chosen_piece.color
                                        {
                                            possible_moves.push(position_converter(row, column - i));
                                        }

                                        else {
                                            possible_moves.push(position_converter(row, column - i + 1));
                                        }

                                        is_left_done = true;
                                    }

                                    None => possible_moves.push(position_converter(row, column - i))
                                }


                                if row + i <= 7 && !is_down_done
                                {
                                let current_targeted_piece: &Option<Piece> = &self.gameboard[row + i][column];

                                match current_targeted_piece
                                {
                                    Some(piece) => { if piece.color != chosen_piece.color
                                        {
                                            possible_moves.push(position_converter(row + i, column));
                                        }

                                        else {
                                            possible_moves.push(position_converter(row + i - 1, column));
                                        }

                                        is_down_done = true;
                                    }

                                    None => possible_moves.push(position_converter(row + i, column))
                                }


                                } 

                                if row - i >= 0 && !is_up_done
                            {
                                let current_targeted_piece: &Option<Piece> = &self.gameboard[row - i][column];

                                match current_targeted_piece
                                {
                                    Some(piece) => { if piece.color != chosen_piece.color
                                        {
                                            possible_moves.push(position_converter(row - i, column));
                                        }

                                        else {
                                            possible_moves.push(position_converter(row - i + 1, column));
                                        }

                                        is_left_done = true;
                                    }

                                    None => possible_moves.push(position_converter(row - i, column))
                                }
                                
                            }
                            }

                                }
                            }
                            }

            }

            else if chosen_piece.what_type == "King"
            {
                let mut maximal_range_of_king: Vec<String> = Vec::new();
                let mut new_gameboard_to_be_scanned = &self.gameboard;

                if !chosen_piece.has_moved
                {
                    let rook1: Option<Piece> = &self.gameboard[row][column + 3];
                    let rook2: Option<Piece> = &self.gameboard[row][column + 4];


                    if chosen_piece.color == "White"
                    {
                    
                        match rook1
                        {
                            Some(piece) => {if piece.what_type == "Rook" {
                                if !piece.has_moved
                                {
                                    maximal_range_of_king.push()
                                }
                            }}
                        }
                    }
                }

                if row > 0
                {   
                    let current_targeted_piece: &Option<Piece> = &self.gameboard[row - 1][column];
                        
                    match current_targeted_piece
                    {

                        Some(piece) => {if piece.color != chosen_piece.color {
                            maximal_range_of_king.push(position_converter(row - 1, column));
                            new_gameboard_to_be_scanned[row - 1][column].color = chosen_piece.color;
                        }}

                        None => maximal_range_of_king.push(position_converter(row - 1, column -1))
                    } 
                    if column > 0
                    {
                        let current_targeted_piece: &Option<Piece> = &self.gameboard[row - 1][column - 1];
                        
                        match current_targeted_piece
                        {

                            Some(piece) => {if piece.color != chosen_piece.color {
                                maximal_range_of_king.push(position_converter(row - 1, column -1));
                                new_gameboard_to_be_scanned[row - 1][column - 1].color = chosen_piece.color;
                            }}

                            None => maximal_range_of_king.push(position_converter(row - 1, column -1))
                        }   
                        
                    }

                    if column < 7
                    {
                        let current_targeted_piece: &Option<Piece> = &self.gameboard[row - 1][column + 1];
                        
                        match current_targeted_piece
                        {

                            Some(piece) => {if piece.color != chosen_piece.color {
                                maximal_range_of_king.push(position_converter(row - 1, column + 1));
                                new_gameboard_to_be_scanned[row - 1][column + 1].color = chosen_piece.color;
                            }}

                            None => maximal_range_of_king.push(position_converter(row - 1, column + 1))
                        } 
                    }

                }

                if row < 7
                {   
                    let current_targeted_piece: &Option<Piece> = &self.gameboard[row + 1][column];
                        
                    match current_targeted_piece
                    {

                        Some(piece) => {if piece.color != chosen_piece.color {
                            maximal_range_of_king.push(position_converter(row + 1, column));
                            new_gameboard_to_be_scanned[row - 1][column].color = chosen_piece.color;
                        }}

                        None => maximal_range_of_king.push(position_converter(row + 1, column -1))
                    } 
                    if column > 0
                    {
                        let current_targeted_piece: &Option<Piece> = &self.gameboard[row + 1][column - 1];
                        
                        match current_targeted_piece
                        {

                            Some(piece) => {if piece.color != chosen_piece.color {
                                maximal_range_of_king.push(position_converter(row + 1, column - 1));
                                new_gameboard_to_be_scanned[row - 1][column - 1].color = chosen_piece.color;
                            }}

                            None => maximal_range_of_king.push(position_converter(row + 1, column - 1))
                        }   
                        
                    }

                    if column < 7
                    {
                        let current_targeted_piece: &Option<Piece> = &self.gameboard[row + 1][column + 1];
                        
                        match current_targeted_piece
                        {

                            Some(piece) => {if piece.color != chosen_piece.color {
                                maximal_range_of_king.push(position_converter(row + 1, column + 1));
                                new_gameboard_to_be_scanned[row - 1][column + 1].color = chosen_piece.color;
                            }}

                            None => maximal_range_of_king.push(position_converter(row + 1, column + 1))
                        } 
                    }

                }

                if column > 0
                    {
                        let current_targeted_piece: &Option<Piece> = &self.gameboard[row][column - 1];
                        
                        match current_targeted_piece
                        {

                            Some(piece) => {if piece.color != chosen_piece.color {
                                maximal_range_of_king.push(position_converter(row, column - 1));
                                new_gameboard_to_be_scanned[row][column - 1].color = chosen_piece.color;
                            }}

                            None => maximal_range_of_king.push(position_converter(row, column - 1))
                        }   
                        
                    }
                
                if column < 7
                    {
                        let current_targeted_piece: &Option<Piece> = &self.gameboard[row][column + 1];
                        
                        match current_targeted_piece
                        {

                            Some(piece) => {if piece.color != chosen_piece.color {
                                maximal_range_of_king.push(position_converter(row, column + 1));
                                new_gameboard_to_be_scanned[row][column + 1].color = &chosen_piece.color;
                            }}

                            None => maximal_range_of_king.push(position_converter(row, column + 1))
                        }   
                        
                    }

                }

                let maximal_range_of_opposite_king = get_opposite_kings_range(&self, find_opposite_king(&self, &chosen_piece));

                for moves in maximal_range_of_opposite_king
                {
                    if maximal_range_of_king.contains(moves)
                    {
                        if let Some(index) = maximal_range_of_king.iter().position(|&x| x == moves) {
                            maximal_range_of_king.remove(index);
                        }
                    }
                }


                for every_row in new_gameboard_to_be_scanned
                {
                    for every_column in every_row
                    {
                        if maximal_range_of_king.contains(new_gameboard_to_be_scanned[every_row][every_column])
                        {
                            if let Some(index) = maximal_range_of_king.iter().position(|moves| moves == new_gameboard_to_be_scanned[every_row][every_column])
                            {
                                maximal_range_of_king.remove(index);
                            }
                        }
                            
                    }
                    
                }

                for every_move in maximal_range_of_opposite_king
                {
                    possible_moves.push(every_move)
                }
                            
            possible_moves
        }

        else {
            None
        }

            
    }
}



pub fn new() -> Game {

    let mut chessboard: [[Option<Piece>; 8]; 8] = [[None; 8]; 8];

    for column in 0..8
    {

        if column == 0 || column == 7
        {
        
            chessboard[0][column] = Piece::new(&position_converter(0 as u8, column as u8), "Black", "Rook", false); 
            chessboard[1][column] = Piece::new(&position_converter(1 as u8, column as u8), "Black", "Pawn", None); // At start, in every column there is two pawns and two different pieces on both sides which respectively are black and white. That's why we have the numbers like 0, 1, 6 ,7 (one special piece and pawn at one end and vice versa at the other end of the table)
            chessboard[6][column] = Piece::new(&position_converter(6 as u8, column as u8), "White", "Pawn", None);
            chessboard[7][column] = Piece::new(&position_converter(7 as u8, column as u8), "White", "Rook", false); //On contrary to rooks, pawns have the variable 'has_mode' on None by default. This variable is just for checking if rooks or kings have moved before castling

        }

        else if column == 1 || column == 6
        {
        
            chessboard[0][column] = Piece::new(&position_converter(0 as u8, column as u8), "Black", "Knight", None); 
            chessboard[1][column] = Piece::new(&position_converter(1 as u8, column as u8), "Black", "Pawn", None); 
            chessboard[6][column] = Piece::new(&position_converter(6 as u8, column as u8), "White", "Pawn", None);
            chessboard[7][column] = Piece::new(&position_converter(7 as u8, column as u8), "White", "Knight", None); 
        }

        else if column == 2 || column == 5
        {
        
            chessboard[0][column] = Piece::new(&position_converter(0 as u8, column as u8), "Black", "Bishop", None); 
            chessboard[1][column] = Piece::new(&position_converter(1 as u8, column as u8), "Black", "Pawn", None); 
            chessboard[6][column] = Piece::new(&position_converter(6 as u8, column as u8), "White", "Pawn", None);
            chessboard[7][column] = Piece::new(&position_converter(7 as u8, column as u8), "White", "Bishop", None); 
        }

        else if column == 3
        {
        
            chessboard[0][column] = Piece::new(&position_converter(0 as u8, column as u8), "Black", "Queen", None); 
            chessboard[1][column] = Piece::new(&position_converter(1 as u8, column as u8), "Black", "Pawn", None); 
            chessboard[6][column] = Piece::new(&position_converter(6 as u8, column as u8), "White", "Pawn", None);
            chessboard[7][column] = Piece::new(&position_converter(7 as u8, column as u8), "White", "Queen", None); 
        }

        else if column == 3
        {
        
            chessboard[0][column] = Piece::new(&position_converter(0 as u8, column as u8), "Black", "King", false); 
            chessboard[1][column] = Piece::new(&position_converter(1 as u8, column as u8), "Black", "Pawn", None); 
            chessboard[6][column] = Piece::new(&position_converter(6 as u8, column as u8), "White", "Pawn", None);
            chessboard[7][column] = Piece::new(&position_converter(7 as u8, column as u8), "White", "King", false); 
        }
    }
    
    let new_gameboard: Game = Game { gameboard: chessboard, current_state: InProgress };
    new_gameboard
                
}
fn position_converter(x: u8, y: u8) -> String 
{

    let row: char = char::from_u32(x as u32 + 65).unwrap(); // 65 is A's ASCII chart number, in Rust chars can also be modified as if they are numbers and there is a special chart (ASCII chart) for that
    let column: char = char::from_u32(y as u32 + 65).unwrap();

    let position: String = [row, column].iter().collect();

    position
}

fn reverse_position_converter(position: String) -> [u8; 2]
{   

    let column = position[0]..as_bytes()[0] - 65; 
    let row: u8 = position[1].parse().unwrap();

    let coordinates: [u8; 2] = [row, column];
    coordinates
}

fn find_opposite_king(&current_game: Game, main_chosen_piece: Piece) -> Piece
{
    for rows in 0..8
    {
        for columns in 0..8{

            let current_targeted_piece = &current_game.gameboard[rows][columns];
            match current_targeted_piece
            {
                Some(piece) => {if current_targeted_piece.what_type == "King" && current_targeted_piece.color != main_chosen_piece.color {

                    piece
                    
                }}

                None => ()
            }

        }
        
    }
}

fn get_opposite_kings_range(&current_game: Game, opposite_king: Piece) -> Vec<String>
{
    let maximal_range_of_opposite_king: Vec<String> = Vec::new();
    let (row, column): (u8, u8) = reverse_position_converter(Piece.position).iter().collect();

    if row > 0
    {   
        let current_targeted_piece: &Option<Piece> = &current_game.gameboard[row - 1][column];
            
        match current_targeted_piece
        {

            Some(piece) => {if piece.color != chosen_piece.color {
                maximal_range_of_opposite_king.push(position_converter(row - 1, column));
            }}

            None => maximal_range_of_opposite_king.push(position_converter(row - 1, column -1))
        } 
        if column > 0
        {
            let current_targeted_piece: &Option<Piece> = &current_game.gameboard[row - 1][column - 1];
            
            match current_targeted_piece
            {

                Some(piece) => {if piece.color != chosen_piece.color {
                    maximal_range_of_opposite_king.push(position_converter(row - 1, column -1));
                }}

                None => maximal_range_of_opposite_king.push(position_converter(row - 1, column -1))
            }   
            
        }

        if column < 7
        {
            let current_targeted_piece: &Option<Piece> = &current_game.gameboard[row - 1][column + 1];
            
            match current_targeted_piece
            {

                Some(piece) => {if piece.color != chosen_piece.color {
                    maximal_range_of_opposite_king.push(position_converter(row - 1, column -1));
                }}

                None => maximal_range_of_opposite_king.push(position_converter(row - 1, column + 1))
            } 
        }

    }

    if row < 7
    {   
        let current_targeted_piece: &Option<Piece> = &self.gameboard[row + 1][column];
            
        match current_targeted_piece
        {

            Some(piece) => {if piece.color != chosen_piece.color {
                maximal_range_of_opposite_king.push(position_converter(row + 1, column));
            }}

            None => maximal_range_of_opposite_king.push(position_converter(row + 1, column -1))
        } 
        if column > 0
        {
            let current_targeted_piece: &Option<Piece> = &self.gameboard[row + 1][column - 1];
            
            match current_targeted_piece
            {

                Some(piece) => {if piece.color != chosen_piece.color {
                    maximal_range_of_opposite_king.push(position_converter(row + 1, column - 1));
                }}

                None => maximal_range_of_opposite_king.push(position_converter(row + 1, column - 1))
            }   
            
        }

        if column < 7
        {
            let current_targeted_piece: &Option<Piece> = &self.gameboard[row + 1][column + 1];
            
            match current_targeted_piece
            {

                Some(piece) => {if piece.color != chosen_piece.color {
                    maximal_range_of_opposite_king.push(position_converter(row + 1, column + 1));
                }}

                None => maximal_range_of_opposite_king.push(position_converter(row + 1, column + 1))
            } 
        }

    }

    if column > 0
        {
            let current_targeted_piece: &Option<Piece> = &self.gameboard[row][column - 1];
            
            match current_targeted_piece
            {

                Some(piece) => {if piece.color != chosen_piece.color {
                    maximal_range_of_opposite_king.push(position_converter(row, column - 1));
                    
                }}

                None => maximal_range_of_opposite_king.push(position_converter(row, column - 1))
            }   
            
        }
    
    if column < 7
        {
            let current_targeted_piece: &Option<Piece> = &self.gameboard[row][column + 1];
            
            match current_targeted_piece
            {

                Some(piece) => {if piece.color != chosen_piece.color {
                    maximal_range_of_opposite_king.push(position_converter(row, column + 1));
                }}

                None => maximal_range_of_opposite_king.push(position_converter(row, column + 1))
            }   
            
        }
    maximal_range_of_opposite_king
}
