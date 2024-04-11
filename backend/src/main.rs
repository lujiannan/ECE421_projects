mod connect4;
use connect4::{Board, Player, State};
use std::io::{self, Write};


// toot otto
mod toot_otto;
use toot_otto::{Board as toot_Board, Player as toot_Playser, State as toot_State, Piece};



fn main() {
    let mut board = toot_Board::new(4, 6);
    board.display();
    board.insert_piece(3, Piece::O);
    board.display();

}








fn get_user_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_owned()
}

fn interface(play_against_computer: bool) {
    let mut board = Board::new(6, 7); // Assuming a standard Connect Four board size

    loop {
        // Display the board and the current turn
        board.display();
        println!("Current turn: {:?}", board.current_turn);

        if matches!(board.current_turn, Player::Red) || !play_against_computer {
            // Human turn
            let col_input = get_user_input("Enter column (0-6) to drop your disc: ");
            let col: usize = match col_input.parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Invalid input. Please enter a column number between 0 and 6.");
                    continue;
                },
            };

            // Insert disc for human
            if let Err(e) = board.insert_disc(col) {
                println!("Error: {}", e);
                continue;
            }
        } else {
            // Computer's turn (assuming the computer is Yellow and play_against_computer is true)
            println!("Computer's turn.");
            if let Err(e) = board.computer_move() {
                println!("Error: {}", e);
                continue;
            }
        }

        // Check game state after each move
        match board.state {
            State::Won(player) => {
                board.display();
                println!("Player {:?} wins!", player);
                break;
            },
            State::Draw => {
                board.display();
                println!("The game is a draw!");
                break;
            },
            State::Running => {} // Continue the game
        }
    }
}

// fn main() {
//     let mode_input = get_user_input("Do you want to play against (1) another player or (2) the computer? Enter 1 or 2: ");
//     let play_against_computer = mode_input == "2";
    
//     interface(play_against_computer);
// }
