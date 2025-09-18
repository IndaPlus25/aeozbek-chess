
fn main() {
   
}

struct Piece {

        position: String,
        color: String,
        what_type: String,

    }

impl Piece {

    fn new(position: &str, color: &str, what_type: &str) -> Self {

        Piece { position: position.to_string(), color: color.to_string(), what_type: what_type.to_string()}
    }
}

struct Game {

    gameboard: Vec<Vec<Piece>>,
        
}


pub fn new() -> Game {

    let mut chessboard: Vec<Vec<Piece>> = (0..8).map(|_| Vec::new()).collect();

    for row in 0..8 {

        for column in 0..8 {

            if row == 0 {

                if column == 0|| column == 7 {

                    let rook: Piece = Piece::new(&position_converter(row, column), "Black", "Rook");

                    chessboard[row].push(rook);
                }

                else if column == 1 || column == 6 {

                    let knight: Piece = Piece::new(&position_converter(row, column), "Black", "Knight");

                    chessboard[row].push(knight);

                }

                else if column == 2 || column == 5 {

                    let bishop: Piece = Piece::new(&position_converter(row, column), "Black","Bishop");

                    chessboard[row].push(bishop);

                }

                else if column == 4 {

                    let king: Piece = Piece::new(&position_converter(row, column), "Black","King");

                    chessboard[row].push(king);

                }
                
                else {

                    let queen: Piece = Piece::new(&position_converter(row, column), "Black","Queen");

                    chessboard[row].push(queen);

                }


            }
            
            if row == 1 {

                let pawn: Piece = Piece::new(&position_converter(row, column), "White","Pawn");

                chessboard[row].push(pawn);

            }

            if row == 7 {

                if column == 0 || column == 7 {

                    let rook: Piece = Piece::new(&position_converter(row, column), "White", "Rook");

                    chessboard[row].push(rook);
                }

                else if column == 1 || column == 6 {

                    let knight: Piece = Piece::new(&position_converter(row, column), "White", "Knight");

                    chessboard[row].push(knight);

                }

                else if column == 2 || column == 5 {

                    let bishop: Piece = Piece::new(&position_converter(row, column), "White","Bishop");

                    chessboard[row].push(bishop);

                }

                else if column == 4 {

                    let king: Piece = Piece::new(&position_converter(row, column), "White","King");

                    chessboard[row].push(king);

                }
                
                else {

                    let queen: Piece = Piece::new(&position_converter(row, column), "White","Queen");

                    chessboard[row].push(queen);

                }


            }
            
            if row == 6 {

                let pawn: Piece = Piece::new(&position_converter(row, column), "White","Pawn");

                chessboard[row].push(pawn);

            }
            
            
        }
    }

    let gameboard: Game = Game { gameboard: chessboard };

    gameboard
}

fn position_converter(x: usize, y: i32) -> String {

    let row: char = char::from_u32(x as u32 - 97).unwrap();
    let column: char = char::from_u32(y as u32 - 97).unwrap();


    let position: String = [row, column].iter().collect();

    position
}