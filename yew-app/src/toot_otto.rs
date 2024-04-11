use serde::{Serialize, Deserialize};

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


    pub fn display(&self) {
        for row in &self.grid {
            for cell in row {
                match cell {
                    Cell::Empty => print!(" . "),
                    Cell::Occupied(piece) => match piece {
                        Piece::T => print!(" T "),
                        Piece::O => print!(" O "),
                    },
                }
            }
            println!();
        }
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

                // Check if this move wins the game
                if let Some(winner) = self.check_win(row, col) {
                    self.state = State::Won(winner);
                    return Ok(());  // End the game since there's a winner
                }

                // Check if the game is a draw
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


    // Switch the current player's turn
    pub fn switch_turn(&mut self) {
        self.current_turn = match self.current_turn {
            Player::Toot => Player::Otto,
            Player::Otto => Player::Toot,
        };
    }

    
    // Check if the last move resulted in a win
// Check if the last move resulted in a win
    pub fn check_win(&self, last_row: usize, last_col: usize) -> Option<Player> {
        // Convert the entire row to a string representation
        let row_string: String = self.grid[last_row].iter().map(|cell| {
            match cell {
                Cell::Occupied(Piece::T) => 'T',
                Cell::Occupied(Piece::O) => 'O',
                _ => '.',
            }
        }).collect();

        // Define the target sequences based on the current player's turn
        // let target_sequence = match self.current_turn {
        //     Player::Toot => "TOOT", 
        //     Player::Otto => "OTTO",
        // };


            // Check for both winning sequences
        if row_string.contains("TOOT") {
            return Some(Player::Toot);
        }
        if row_string.contains("OTTO") {
            return Some(Player::Otto);
        }

    None
    }

    // Check if the game is a draw (the board is full)
    pub fn is_draw(&self) -> bool {
        self.grid.iter().all(|row| row.iter().all(|cell| matches!(cell, Cell::Occupied(_))))
    }
}
