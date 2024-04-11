#[macro_use] extern crate rocket;

use serde::{Serialize, Deserialize};
use rocket::serde::json::Json;

// Assuming your connect4.rs is in the same src directory and declared as a module
mod connect4;
use connect4::{Board, Player, State};

// Define a response structure for the game state
#[derive(Serialize)]
struct GameState {
    board: Board,
}

#[post("/start")]
fn start() -> Json<GameState> {
    let board = Board::new(6, 7); // Initialize a 6x7 board
    Json(GameState { board })
}

#[post("/move", data = "<game_move>")]
fn make_move(game_move: Json<(usize, Board)>) -> Json<Result<GameState, &'static str>> {
    let (col, mut board) = game_move.into_inner();
    match board.insert_disc(col) {
        Ok(_) => Json(Ok(GameState { board })),
        Err(e) => Json(Err(e)),
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![start, make_move])
}
