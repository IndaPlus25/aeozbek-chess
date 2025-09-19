struct Piece {

        position: String,
        color: String,
        what_type: String,
        has_moved: Option<bool>

    }

impl Piece {

    fn new(position: &str, color: &str, what_type: &str, has_moved: Option<bool>) -> Self {

        Piece { position: position.to_string(), color: color.to_string(), what_type: what_type.to_string(), has_moved: has_moved}
    }
}

enum GameState {

    InProgress,
    Check,
    GameOver,
    Checkmate,
    DeadPosition,
}

struct Game {

    gameboard: [[Option<Piece>; 8]; 8],
    current_state: GameState
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

    let row: char = char::from_u32(x as u32 + 65).unwrap();
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