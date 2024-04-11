mod connect4;
use connect4::{Board, Player, State};
use std::io::{self, Write};

fn main() {
    let mut board = Board::new(6, 7); // Assuming a standard Connect Four board size

    loop {

        // display the board and the current turn
        board.display();
        println!("Current turn: {:?}", board.current_turn);
        print!("Enter column (0-6) to drop your disc: ");
        io::stdout().flush().unwrap();

        // get the column
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let col: usize = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a column number between 0 and 6.");
                continue;
            },
        };

        // insert disc and check the state
        match board.insert_disc(col) {
            Ok(_) => {
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
            },
            Err(e) => println!("Error: {}", e),
        }
    }
}

// #[macro_use] extern crate rocket;

// #[get("/")]
// fn index() -> &'static str {
//     "Hello, world from Rocket!"
// }

// #[launch]
// fn rocket() -> _ {
//     rocket::build().mount("/", routes![index])
// }

