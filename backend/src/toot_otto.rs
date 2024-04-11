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
    grid: Vec<Vec<Cell>>,
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
    pub fn insert_piece(&mut self, col: usize, piece: Piece) -> Result<(), &'static str> {
        if col >= self.cols {
            return Err("Column out of bounds");
        }

        // Attempt to place the piece in the lowest empty cell in the specified column
        for row in (0..self.rows).rev() {
            if let Cell::Empty = self.grid[row][col] {
                self.grid[row][col] = Cell::Occupied(piece);
                // Check if this move wins the game or results in a draw
                if self.check_win(row, col) {
                    self.state = State::Won(self.current_turn);
                } else if self.is_draw() {
                    self.state = State::Draw;
                }
                self.switch_turn(); // Switch the turn to the other player
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
    pub fn check_win(&self, last_row: usize, last_col: usize) -> bool {
        // Implement win checking logic here
        // You'll need to check for sequences of "TOOT" or "OTTO"
        false
    }

    // Check if the game is a draw (the board is full)
    pub fn is_draw(&self) -> bool {
        self.grid.iter().all(|row| row.iter().all(|cell| matches!(cell, Cell::Occupied(_))))
    }

    // Example function to insert a piece, you'll need to adapt this to switch between T and O based on player input
    // pub fn insert_piece(&mut self, col: usize, piece: Piece) -> Result<(), &'static str> {
    //     // Insertion logic similar to Connect Four, adapted for TOOT-OTTO pieces
    // }

    // // You'll need a new method to check for TOOT or OTTO patterns specifically
    // fn check_win(&self) -> bool {
    //     // Implement checking logic for TOOT and OTTO sequences
    // }

    // Switch turns, draw checks, etc., can remain largely unchanged, but you may need to adapt them to the new gameplay mechanics
}
