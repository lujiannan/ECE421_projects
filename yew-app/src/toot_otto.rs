use serde::{Serialize, Deserialize};
use rand::Rng; // Import the Rng trait to use random number generation
use rand::seq::SliceRandom;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Piece {
    T,
    O,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Player {
    Toot,
    Otto,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Winner {
    Player(Player),  // To capture which player won
    None,            // No winner yet
    Draw,            // Game is a draw
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Cell {
    Empty,
    Occupied(Piece),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Board {
    pub grid: Vec<Vec<Cell>>,
    pub current_turn: Player,
    pub rows: usize,
    pub cols: usize,
    pub state: State,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum State {
    Running,
    Won(Player),
    Draw,
}

impl Board {
    pub fn new(rows: usize, cols: usize) -> Board {
        Board {
            grid: vec![vec![Cell::Empty; cols]; rows],
            current_turn: Player::Toot, // Start with TOOT player
            rows,
            cols,
            state: State::Running,
            
        }
    }


    // pub fn display(&self) {
    //     for row in &self.grid {
    //         for cell in row {
    //             match cell {
    //                 Cell::Empty => print!(" . "),
    //                 Cell::Occupied(piece) => match piece {
    //                     Piece::T => print!(" T "),
    //                     Piece::O => print!(" O "),
    //                 },
    //             }
    //         }
    //         println!();
    //     }
    // }

    pub fn computer_move(&mut self) -> Result<(), &'static str> {
        let mut rng = rand::thread_rng();
        let mut attempts = 0;
        loop {
            let col = rng.gen_range(0..self.cols);
            let pieces = [Piece::T, Piece::O]; // Array of pieces
            let piece = *pieces.choose(&mut rng).expect("Failed to select a random piece");

            if let Ok(_) = self.insert_piece(col, piece) {
                println!("Computer placed piece on column {}", col + 1);
                break;
            }
            attempts += 1;
            if attempts > 100 { // Prevents an infinite loop
                return Err("Failed to make a move after multiple attempts.");
            }
        }
        Ok(())
    }

    // Hard strategy move focusing around a given column
    pub fn computer_move_hard(&mut self, given_col: usize) -> Result<(), &'static str> {
        let mut rng = rand::thread_rng();
        let mut attempts = 0;
        loop {
            let choices = [
                given_col,
                given_col.saturating_add(1),
                given_col.saturating_sub(1),
            ];
            let col = *choices.choose(&mut rng).unwrap_or(&given_col); // Ensures a fallback to the given column

            let pieces = [Piece::T, Piece::O]; // Array of pieces
            let piece = *pieces.choose(&mut rng).expect("Failed to select a random piece");

            if let Ok(_) = self.insert_piece(col, piece) {
                println!("Computer placed piece on column {}", col + 1);
                break;
            }
            attempts += 1;
            if attempts > 100 {
                return Err("Failed to make a move after multiple attempts.");
            }
        }
        Ok(())
    }


    // Insert a piece into the specified column
    // Insert a piece into the specified column
    pub fn insert_piece(&mut self, col: usize, piece: Piece) -> Result<(), &'static str> {
        if col >= self.cols {
            return Err("Column out of bounds");
        }

        // Attempt to place the piece in the lowest empty cell in the specified column
        for row in (0..self.rows).rev() {
            if matches!(self.grid[row][col], Cell::Empty) {
                self.grid[row][col] = Cell::Occupied(piece);

                match self.check_win(row, col) {
                    Some(Winner::Player(player)) => {
                        self.state = State::Won(player);
                        return Ok(());  // End the game since there's a winner
                    },
                    Some(Winner::Draw) => {
                        self.state = State::Draw;
                        return Ok(());  // End the game since it's a draw (toot and otto win same time)
                    },
                    _ => {} // Continue the game if there is no winner
                }
                

                // Check if the game is a draw (board full condition)
                if self.is_draw() {
                    self.state = State::Draw;
                    return Ok(());  // End the game since it's a draw
                }

                // If no win or draw, switch turns
                self.switch_turn();
                return Ok(());
            }
        }

        Err("Column is full")
    }

    pub fn predict_piece(&self, col: usize) -> Option<(usize, usize)> {
        if col >= self.cols {
            return None;
        }

        for row in (0..self.rows).rev() {
            if matches!(self.grid[row][col], Cell::Empty) {
                return Some((row, col));
            }
        }

        return None;
    } 


    // Switch the current player's turn
    pub fn switch_turn(&mut self) {
        self.current_turn = match self.current_turn {
            Player::Toot => Player::Otto,
            Player::Otto => Player::Toot,
        };
    }

    
    
    fn row_to_string(&self, row: usize) -> String {
        self.grid[row].iter().map(|cell| {
            match cell {
                Cell::Occupied(Piece::T) => 'T',
                Cell::Occupied(Piece::O) => 'O',
                _ => '.',
            }
        }).collect()
    }

    // Generates a string from a column for checking win conditions
    fn col_to_string(&self, col: usize) -> String {
        self.grid.iter().map(|row| {
            match row[col] {
                Cell::Occupied(Piece::T) => 'T',
                Cell::Occupied(Piece::O) => 'O',
                _ => '.',
            }
        }).collect()
    }

    // Generates a string from the major diagonal
    fn major_diag_to_string(&self, start_row: usize, start_col: usize) -> String {
        let mut result = String::new();
        let mut row = start_row;
        let mut col = start_col;
        while row < self.rows && col < self.cols {
            match self.grid[row][col] {
                Cell::Occupied(Piece::T) => result.push('T'),
                Cell::Occupied(Piece::O) => result.push('O'),
                _ => result.push('.'),
            }
            row += 1;
            col += 1;
        }
        result
    }

    // Generates a string from the minor diagonal
    fn minor_diag_to_string(&self, start_row: usize, start_col: usize) -> String {
        let mut result = String::new();
        let mut row = start_row;
        let mut col = start_col;
        while row < self.rows && col < self.cols {
            match self.grid[row][col] {
                Cell::Occupied(Piece::T) => result.push('T'),
                Cell::Occupied(Piece::O) => result.push('O'),
                _ => result.push('.'),
            }
            if col == 0 {  // Prevent underflow by breaking the loop if col is zero
                break;
            }
            row += 1;
            col -= 1;  // Safe decrement as we check before this operation
        }
        result
    }


    // Check if the last move resulted in a win
    pub fn check_win(&self, last_row: usize, last_col: usize) -> Option<Winner> {
        let row_string = self.row_to_string(last_row);
        let col_string = self.col_to_string(last_col);

        let start_major_diag = usize::min(last_row, last_col);
        let major_diag_string = self.major_diag_to_string(last_row - start_major_diag, last_col - start_major_diag);

        let start_minor_diag = usize::min(last_row, self.cols - 1 - last_col);
        let minor_diag_string = self.minor_diag_to_string(last_row - start_minor_diag, last_col + start_minor_diag);

        let mut toot_win = false;
        let mut otto_win = false;
        // Check all strings for win conditions
        for string in &[row_string, col_string, major_diag_string, minor_diag_string] {
            if string.contains("TOOT") {
                toot_win = true;
            }
            if string.contains("OTTO") {
                otto_win = true;
            }
        }

        // Check for draw by simultaneous win condition
        if toot_win && otto_win {
            return Some(Winner::Draw); // Correctly return a Winner enum for draw
        } else if toot_win {
            return Some(Winner::Player(Player::Toot)); // Correctly return Winner::Player enum variant for Toot win
        } else if otto_win {
            return Some(Winner::Player(Player::Otto)); // Correctly return Winner::Player enum variant for Otto win
        }
        

        // No win found
        None
    }


    // Check if the game is a draw (the board is full)
    pub fn is_draw(&self) -> bool {
        self.grid.iter().all(|row| row.iter().all(|cell| matches!(cell, Cell::Occupied(_))))
    }
}
