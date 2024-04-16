mod connect4;
use connect4::{Board, Player, State};

mod toot_otto;
use toot_otto::{Board as TootBoard, Player as TootPlayer, State as TootState, Piece};

use std::io::{self, Write};

fn main() {
    println!("Choose your game:");
    println!("1: Connect Four");
    println!("2: TOOT-OTTO");

    let choice = get_user_input("Enter choice (1 for Connect Four, 2 for TOOT-OTTO): ");

    match choice.as_str() {
        "1" => connect_four_interface(),
        "2" => toot_otto_interface(),
        _ => println!("Invalid choice, please restart the program."),
    }
}



fn get_user_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_owned()
}

fn connect_four_interface() {
    println!("Do you want to play against (1) another player or (2) the computer? Enter 1 or 2: ");
    let mode_input = get_user_input("");
    let play_against_computer = mode_input == "2";

    let mut board = Board::new(6, 7); // Standard Connect Four board size

    game_loop(&mut board, play_against_computer);
}

fn game_loop(board: &mut Board, play_against_computer: bool) {
    loop {
        board.display();
        println!("Current turn: {:?}", board.current_turn);

        if !play_against_computer || matches!(board.current_turn, Player::Red) {
            let col_input = get_user_input("Enter column (0-6) to drop your disc: ");
            let col = match col_input.parse::<usize>() {
                Ok(num) if num < 7 => num,
                _ => {
                    println!("Invalid input. Please enter a column number between 0 and 6.");
                    continue;
                },
            };

            if let Err(e) = board.insert_disc(col) {
                println!("Error: {}", e);
                continue;
            }
        } else {
            // Computer's turn
            println!("Computer's turn.");
            if let Err(e) = board.computer_move() {
                println!("Error: {}", e);
                continue;
            }
        }

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








////////////////////// toot otto


fn toot_otto_interface() {
    let mut board = TootBoard::new(4, 6); // Assuming a standard size for TOOT-OTTO
    let mut play_against_computer = false;  // Default to two player mode
    println!("Do you want to play against (1) another player or (2) the computer? Enter 1 or 2: ");
    let mode_input = get_user_input("");
    play_against_computer = mode_input == "2";

    toot_otto_game_loop(&mut board, play_against_computer);
}

fn toot_otto_game_loop(board: &mut TootBoard, play_against_computer: bool) {
    loop {
        board.display();

        let current_player = if matches!(board.current_turn, TootPlayer::Toot) {
            "TOOT"
        } else {
            "OTTO"
        };
        println!("Current turn for: {}", current_player);

        // Get player's piece choice
        let piece_input = get_user_input("Choose your piece (T or O): ");
        let piece = match piece_input.as_str() {
            "T" | "t" => Piece::T,
            "O" | "o" => Piece::O,
            _ => {
                println!("Invalid piece. Please choose 'T' or 'O'.");
                continue;
            }
        };

        // Get player's column choice
        let col_input = get_user_input("Enter column number to place your piece: ");
        let col = match col_input.parse::<usize>() {
            Ok(num) if num < board.cols => num,
            _ => {
                println!("Invalid input. Please enter a valid column number.");
                continue;
            }
        };

        // Attempt to insert the piece into the board
        if let Err(e) = board.insert_piece(col, piece) {
            println!("Error: {}", e);
            continue;
        }

        // Check game state after each move
        match board.state {
            TootState::Won(player) => {
                board.display();
                println!("Player {:?} wins!", player);
                break;
            },
            TootState::Draw => {
                board.display();
                println!("The game is a draw!");
                break;
            },
            TootState::Running => {} // Continue the game
        }
    }
}
