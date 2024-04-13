use rand::distributions::{Distribution, WeightedIndex};
use serde::{Serialize, Deserialize};
use rand::Rng; // Import the Rng trait to use random number generation
use rand::seq::SliceRandom;

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
    pub grid: Vec<Vec<Cell>>,
    pub current_turn: Player,
    pub rows: usize,
    pub cols: usize,
    pub state: State,
    pub last_move: Option<(usize, usize)>, // Track the last move as (row, col)
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
    // pub fn display(&self) {
    //     for row in &self.grid {
    //         for cell in row {
    //             match cell {
    //                 Cell::Empty => print!(" . "),
    //                 Cell::Occupied(player) => match player {
    //                     Player::Red => print!(" R "),
    //                     Player::Yellow => print!(" Y "),
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
            if let Ok(_) = self.insert_disc(col) {
                println!("Computer placed on column {}", col + 1);
                break;
            }
            attempts += 1;
            if attempts > 100 { // Just to prevent an infinite loop
                return Err("Failed to make a move after multiple attempts.");
            }
        }
        Ok(())
    }

    pub fn computer_move_hard(&mut self, given_col: usize) -> Result<(), &'static str> {
        let mut rng = rand::thread_rng();
        let offsets = [-1, 0, 1]; // possible offsets
        let weights = [33, 34, 33]; // weights for each offset
        let dist = WeightedIndex::new(&weights).unwrap(); // distribution for the offsets (given the weights)
        let mut attempts = 0;
        loop {
            //generic 'hard' strategy: pick a column near the last move
            let offset = offsets[dist.sample(&mut rng)];
            let col = (given_col as isize + offset).clamp(0, self.cols as isize - 1) as usize;
            if let Ok(_) = self.insert_disc(col) {
                println!("Computer placed on column {}", col + 1);
                break;
            }
            attempts += 1;
            if attempts > 100 { // Just to prevent an infinite loop
                return Err("Failed to make a move after multiple attempts.");
            }
        }
        Ok(())
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

    pub fn predict_disc(&self, col: usize) -> Option<(usize, usize)>{
        if col >= self.cols {
            return None;
        }
    
        for row in (0..self.rows).rev() {
            if let Cell::Empty = self.grid[row][col] {
                return Some((row, col));
            }
        }
        return None;
    }
    

    // Switch the current player's turn
    pub fn switch_turn(&mut self) {
        self.current_turn = match self.current_turn {
            Player::Red => Player::Yellow,
            Player::Yellow => Player::Red,
        };
    }

    // if the board is full it is a draw.
    fn is_draw(&self) -> bool {
        self.grid.iter().all(|row| row.iter().all(|&cell| cell != Cell::Empty))
    }

    // Check the board for a winning condition. 
    /*
    Since we are tracking the last move. We can check based on the last move
    instead of iterating over the entire board
    We go to the past piece inserted and do a horizonalt, vertical, diagonal check
     */




    pub fn check_win(&self, last_row: usize, last_col: usize) -> bool {
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
    
        // Diagonal Check (Descending from top-left to bottom-right)
        let mut count = 1; // start with the last move
        // Check upwards to the left
        for i in 1..=3 {
            if last_col >= i && last_row >= i && self.grid[last_row - i][last_col - i] == last_player {
                count += 1;
            } else {
                break;
            }
        }
        // Check downwards to the right
        for i in 1..=3 {
            if last_row + i < self.rows && last_col + i < self.cols && self.grid[last_row + i][last_col + i] == last_player {
                count += 1;
            } else {
                break;
            }
        }
        if count >= 4 {
            return true;
        }

        // Diagonal Check (Ascending from bottom-left to top-right)
        count = 1; // reset for the next diagonal check
        // Check downwards to the left
        for i in 1..=3 {
            if last_col >= i && last_row + i < self.rows && self.grid[last_row + i][last_col - i] == last_player {
                count += 1;
            } else {
                break;
            }
        }
        // Check upwards to the right
        for i in 1..=3 {
            if last_row >= i && last_col + i < self.cols && self.grid[last_row - i][last_col + i] == last_player {
                count += 1;
            } else {
                break;
            }
        }
        if count >= 4 {
            return true;
        }

        // If all checks fail then it is not a win
        false
        }
        

        
    
}
