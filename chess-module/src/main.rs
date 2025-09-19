struct Piece 
{

        position: String,
        color: String,
        what_type: String,
        has_moved: Option<bool>

    }

impl Piece 
{

    fn new(position: &str, color: &str, what_type: &str, has_moved: Option<bool>) -> Self {

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
        let which_piece: Option<Piece> = &self.gameboard[row][column];

        if let Some(chosen_piece) = which_piece
        {
            if chosen_piece.what_type == "Pawn" 
            {

                if chosen_piece.color == "Black" && row < 7
                {

                    let piece_at_front: Option<Piece> = Some(&self.gameboard[row + 1][column]);

                    match piece_at_front
                    {
                        Some(_) => (),
                        None => possible_moves.push(position_converter(row, column))
                    }

                    if column < 7
                    {
                        let piece_on_right: Option<Piece> = Some(&self.gameboard[row + 1][column + 1]);

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
                        let piece_on_left: Option<Piece> = (&self.gameboard[row + 1][column - 1]);

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

                    let piece_at_front: Option<Piece> = Some(&self.gameboard[row - 1][column]);

                    match piece_at_front
                    {
                        Some(_) => (),
                        None => possible_moves.push(position_converter(row - 1, column))
                    }

                    if column > 0
                    {
                        let piece_on_right: Option<Piece> = Some(&self.gameboard[row - 1][column + 1]);

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
                        let piece_on_left: Option<Piece> = (&self.gameboard[row - 1][column - 1]);

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
                
            }
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