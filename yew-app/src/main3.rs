mod connect4;

use yew::prelude::*;
use yew::MouseEvent;
use connect4::{Board, Player, State};

const HEIGHT_C4: usize = 6;
const WIDTH_C4: usize = 7;
const HEIGHT_TOOT: usize = 4;
const WIDTH_TOOT: usize = 6;

#[derive(PartialEq, Clone, Copy)]
enum Tile {
    Empty,
    P1(char), // T or O for player 1
    P2(char), // T or O for player 2
}

#[derive(PartialEq, Clone, Copy)]
enum GameType {
    Otto,
    Connect4,
}

#[derive(PartialEq, Clone, Copy)]
enum Difficulty {
    Easy,
    Hard,
}

pub struct Game {
    hover_column: Option<usize>,
    board: Option<Board>,
}

pub enum Msg {
    Click(usize),
    MouseOver(usize),
}

impl Component for Game {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        // game dimension
        Self {
            hover_column: None,
            board: Some(Board::new(HEIGHT_C4, WIDTH_C4)),
        }
    }
    
    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Click(col) => {
                let mut board = self.board.clone().unwrap();
                board.insert_disc(col).ok();
                self.board = Some(board);
                true
            }
            Msg::MouseOver(col) => {
                self.hover_column = Some(col);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let board = self.board.clone().unwrap();
        html! {
            <>
                <h1>{ "Connect Four" }</h1>
                <div>
                    <h3 class="title is-4">{["Status: ", self.get_status_msg(&board).as_str()].concat() }</h3>
                </div>
                <div>
                    {
                        (0..board.cols).map(|col| { // Use `cols` from your board structure
                            html! {
                                <button onclick={ctx.link().callback(move |_event: MouseEvent| Msg::Click(col))}>
                                    { format!("Drop in Col {}", col) }
                                </button>
                            }
                        }).collect::<Html>()
                    }
                </div>
                <div style="font-family: monospace;">
                    {
                        for board.grid.iter().rev().map(|row| { // Access the `grid` directly
                            html! {
                                <div>
                                    { for row.iter().map(|cell| html!{ <span>{ format!("{:?}", cell) }{" "}</span> }) }
                                </div>
                            }
                        })
                    }
                </div>
                <div>
                    {
                        match board.state {
                            State::Won(player) => html! { <p>{ format!("Player {:?} wins!", player) }</p> },
                            State::Draw => html! { <p>{ "The game is a draw!" }</p> },
                            State::Running => html! { <p>{ "Game is in progress..." }</p> },
                        }
                    }
                </div>
            </>
        }
    }
}

impl Game {
    fn get_status_msg(&self, board: &Board) -> String {
        match board.current_turn {
            Player::Red => {
                "Red".to_string()
            }
            Player::Yellow => {
                "Yellow".to_string()
            }
        }
    }
}

// #[function_component(Game)]
// fn app() -> Html {
    // // game dimension
    // let width = 6;
    // let height = 4;
    // let board_ot = use_state(|| vec![vec![Tile::Empty; width]; height]);
    // // player 1 starts the game
    // let current_player = use_state(|| 1);
    // // the winner
    // let winner = use_state(|| -1);
    // // Function to handle a player move
    // let handle_click = {
    //     let board_ot = board_ot.clone();
    //     let current_player = current_player.clone();
    //     let winner = winner.clone();
    //     Callback::from(move |(row, col): (usize, usize)| {
    //         let mut board = (*board_ot).clone();
    //         if board[row][col] == Tile::Empty {
    //             board[row][col] = if *current_player == 1 {
    //                 Tile::P1('T') // For Player 1, 'T'
    //             } else {
    //                 Tile::P2('O') // For Player 2, 'O'
    //             };
    //             board_ot.set(board);
    //             // Switch player turn
    //             current_player.set(if *current_player == 1 { 2 } else { 1 });
    //         }
    //     })
    // };
    // html! {
    //     <div>
    //         <h1>{ "Toot-Otto Game" }</h1>
    //         <div style="display: grid; grid-template-columns: repeat(6, 50px); gap: 5px;">
    //             { for board_ot.iter().enumerate().map(|(row, line)| html! {
    //                 { for line.iter().enumerate().map(|(col, &tile)| {
    //                     let on_click = {
    //                         let row = row;
    //                         let col = col;
    //                         handle_click.reform(move |_| (row, col))
    //                     };

    //                     html! {
    //                         <div style="border: 1px solid black; text-align: center; line-height: 50px; width: 50px; height: 50px;" onclick={on_click}>
    //                             { match tile {
    //                                 Tile::Empty => ' ',
    //                                 Tile::P1(letter) | Tile::P2(letter) => letter,
    //                             }}
    //                         </div>
    //                     }
    //                 })}
    //             })}
    //         </div>
    //         // Add logic to display current player or win conditions
    //     </div>
    // }
// }

fn main() {
    yew::Renderer::<Game>::new().render();
}
