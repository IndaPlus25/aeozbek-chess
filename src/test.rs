use crate::*;

#[test]
fn intialize_gameboard() {

    let new_game = Game::new();

    print_gameboard(&new_game.gameboard, &new_game);

}

 
/* Test result
----------------------------------
R Kn B Q K B Kn R 
P P P P P P P P 
* * * * * * * * 
* * * * * * * * 
* * * * * * * * 
* * * * * * * * 
P P P P P P P P 
R Kn B Q K B Kn R 
----------------------------------

Round 1
Whose turn: White 
Game status: InProgress 
*/

#[test]
fn test_all_piece_moves() {
    let mut new_game = Game::new();
    new_game.make_move("E2".to_string(), "E4".to_string()); // white pawn
    new_game.make_move("E7".to_string(), "E5".to_string()); // black pawn

    new_game.make_move("G1".to_string(), "F3".to_string()); // white knight
    new_game.make_move("B8".to_string(), "C6".to_string()); // black knight

    new_game.make_move("F1".to_string(), "C4".to_string()); // white bishop
    new_game.make_move("F8".to_string(), "B4".to_string()); // black bishop

    new_game.make_move("D1".to_string(), "E2".to_string()); // white queen
    new_game.make_move("D8".to_string(), "F6".to_string()); // black queen

    new_game.make_move("E1".to_string(), "G1".to_string()); // white king (CASTLING)
    new_game.make_move("E8".to_string(), "E7".to_string()); // black king

    new_game.make_move("A2".to_string(), "A4".to_string()); // white pawn
    new_game.make_move("A7".to_string(), "A5".to_string()); // black pawn

    new_game.make_move("A1".to_string(), "A3".to_string()); // white rook
    new_game.make_move("A8".to_string(), "A6".to_string()); // black rook

    print_gameboard(&new_game.gameboard, &new_game);
}

/*----------------------------------
* * ♗ * * * ♘ ♖ 
* ♙ ♙ ♙ ♔ ♙ ♙ ♙ 
♖ * ♘ * * ♕ * * 
♙ * * * ♙ * * * 
♟ ♗ ♝ * ♟ * * * 
♜ * * * * ♞ * * 
* ♟ ♟ ♟ ♛ ♟ ♟ ♟ 
* ♞ ♝ * * ♜ ♚ * 
----------------------------------

Round 15
Whose turn: White 
Game status: InProgress
ok
 */

#[test]
fn promotion_test() {


    let mut new_game = Game::new();
    new_game.make_move("F2".to_string(), "F4".to_string()); 
    new_game.make_move("A7".to_string(), "A5".to_string());

    new_game.make_move("F4".to_string(), "F5".to_string()); 
    new_game.make_move("A5".to_string(), "A4".to_string());

    new_game.make_move("F5".to_string(), "F6".to_string());
    new_game.make_move("A4".to_string(), "A3".to_string()); 
    new_game.make_move("F6".to_string(), "G7".to_string());

    new_game.make_move("A3".to_string(), "B2".to_string()); 
    new_game.make_move("G7".to_string(), "H8".to_string());

    new_game.set_promotion("Queen".to_string());
    
    print_gameboard(&new_game.gameboard, &new_game);

}

/* ----------------------------------
♖ ♘ ♗ ♕ ♔ ♗ ♘ ♛ 
* ♙ ♙ ♙ ♙ ♙ * ♙ 
* * * * * * * * 
* * * * * * * * 
* * * * * * * * 
* * * * * * * * 
♟ ♙ ♟ ♟ ♟ * ♟ ♟ 
♜ ♞ ♝ ♛ ♚ ♝ ♞ ♜ 
----------------------------------

Round 10
Whose turn: Black 
Game status: InProgress */


#[test]
fn check_test() {


    let mut new_game = Game::new();
    new_game.make_move("E2".to_string(), "E4".to_string()); 
    new_game.make_move("D7".to_string(), "D5".to_string());
    new_game.make_move("F1".to_string(), "B5".to_string()); 
    
    print_gameboard(&new_game.gameboard, &new_game);

}

/* Unfortunately does not exactly work

Round 4
Whose turn: Black 
Game status: InProgress
ok
test test::intialize_gameboard ... 
----------------------------------
♖ ♘ ♗ ♕ ♔ ♗ ♘ ♖ 
♙ ♙ ♙ ♙ ♙ ♙ ♙ ♙ 
* * * * * * * * 
* * * * * * * * 
* * * * * * * * 
* * * * * * * * 
♟ ♟ ♟ ♟ ♟ ♟ ♟ ♟ 
♜ ♞ ♝ ♛ ♚ ♝ ♞ ♜ 
----------------------------------*/

fn print_gameboard(board: &[[Option<Piece>; 8]; 8], current_game: &Game) { 
    println!();
    println!("----------------------------------");
    for row in 0..8 {

        for column in 0..8 {

            match &board[row][column] {

                Some(piece) => {

                    match piece.what_type.as_str() {

                        "Rook" => {
                            if piece.color == "White".to_string() {
                                print!("♜ ")
                            }
                            
                            else {
                                print!("♖ ")
                            }
                        },
                        "Knight" => {
                            if piece.color == "White".to_string() {
                                print!("♞ ")
                            }
                            
                            else {
                                print!("♘ ")
                            }
                        },
                        "Bishop" => {
                            if piece.color == "White".to_string() {
                                print!("♝ ")
                            }
                            
                            else {
                                print!("♗ ")
                            }
                        },
                        "Queen" => {
                            if piece.color == "White".to_string() {
                                print!("♛ ")
                            }
                            
                            else {
                                print!("♕ ")
                            }
                        },
                        "King" => {
                            if piece.color == "White".to_string() {
                                print!("♚ ")
                            }
                            
                            else {
                                print!("♔ ")
                            }
                        },
                        "Pawn" => {
                            if piece.color == "White".to_string() {
                                print!("♟ ")
                            }
                            
                            else {
                                print!("♙ ")
                            }
                        },
                        &_ => todo!()
                    }
                }

                None => print!("* ")

            }
        }

        println!();
    }

    println!("----------------------------------");
    println!();
    println!("Round {}", current_game.get_current_turn());
    println!("Whose turn: {} ", current_game.get_whose_turn());
    println!("Game status: {:#?}", current_game.get_game_state());
}


