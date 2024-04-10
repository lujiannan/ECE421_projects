use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Player {
    Red,
    Yellow,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Cell {
    Empty,
    Occupied(Player),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Board {
    grid: Vec<Vec<Cell>>,
    pub current_turn: Player,
    pub rows: usize,
    pub cols: usize,
    pub state: State,
    last_move: Option<(usize, usize)>, // Track the last move as (row, col)
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum State {
    Running,
    Won(Player),
    Draw,
}

impl Board {
    // Initialize a new game board
    pub fn new(rows: usize, cols: usize) -> Board {
        Board {
            grid: vec![vec![Cell::Empty; cols]; rows],
            current_turn: Player::Red,
            rows,
            cols,
            state: State::Running,
            last_move: None,
        }
    }

    // Display the current state of the board
    pub fn display(&self) {
        for row in &self.grid {
            for cell in row {
                match cell {
                    Cell::Empty => print!(" . "),
                    Cell::Occupied(player) => match player {
                        Player::Red => print!(" R "),
                        Player::Yellow => print!(" Y "),
                    },
                }
            }
            println!();
        }
    }

    // Insert a disc into the specified column
    pub fn insert_disc(&mut self, col: usize) -> Result<(), &'static str> {
        if col >= self.cols {
            return Err("Column out of bounds");
        }
    
        for row in (0..self.rows).rev() {
            if let Cell::Empty = self.grid[row][col] {
                self.grid[row][col] = Cell::Occupied(self.current_turn);
                self.last_move = Some((row, col));
                if self.check_win(row, col) {
                    self.state = State::Won(self.current_turn);
                } else if self.is_draw() {
                    self.state = State::Draw;
                }
                self.switch_turn();
                return Ok(());
            }
        }
    
        Err("Column is full")
    }
    

    // Switch the current player's turn
    fn switch_turn(&mut self) {
        self.current_turn = match self.current_turn {
            Player::Red => Player::Yellow,
            Player::Yellow => Player::Red,
        };
    }

    // Check the board for a winning condition. 
    /*
    Since we are tracking the last move. We can check based on the last move
    instead of iterating over the entire board
    We go to the past piece inserted and do a horizonalt, vertical, diagonal check
     */


    fn check_win(&self, last_row: usize, last_col: usize) -> bool {
        let last_player = self.grid[last_row][last_col];
        // Horizontal check
        let mut count = 1;
        for i in 1..=3 {
            if last_col >= i && self.grid[last_row][last_col - i] == last_player {
                count += 1;
            } else {
                break;
            }
        }
        for i in 1..=3 {
            if last_col + i < self.cols && self.grid[last_row][last_col + i] == last_player {
                count += 1;
            } else {
                break;
            }
        }
        if count >= 4 {
            return true;
        }
    
        // Vertical check (only need to check downwards)
        count = 1;
        for i in 1..=3 {
            if last_row + i < self.rows && self.grid[last_row + i][last_col] == last_player {
                count += 1;
            } else {
                break;
            }
        }
        if count >= 4 {
            return true;
        }
    
        // Diagonal checks to be implemented similarly...
    
        false
    }
    

    // if the board is full it is a draw.
    fn is_draw(&self) -> bool {
        self.grid.iter().all(|row| row.iter().all(|&cell| cell != Cell::Empty))
    }
    
}
