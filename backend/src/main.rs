// // At the top of your main.rs
// mod connect4;
// use connect4::{Board, Player, State};

// fn main() {
//     // Create a new game board
//     let mut board = Board::new(6, 7); // Assuming a standard Connect Four board size

    
//     board.insert_disc(3);
//     board.display();

//     /*
//     make a command line. loop to handle game logic to test the module
    
//      */

    
// }

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world from Rocket!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}

