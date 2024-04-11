use yew::prelude::*;

#[derive(PartialEq, Clone, Copy)]
enum Tile {
    Empty,
    P1(char), // T or O for player 1
    P2(char), // T or O for player 2
}

#[function_component(Game)]
fn app() -> Html {
    // game dimension
    let width = 6;
    let height = 4;
    let board_ot = use_state(|| vec![vec![Tile::Empty; width]; height]);
    // player 1 starts the game
    let current_player = use_state(|| 1);
    // the winner
    let winner = use_state(|| -1);
    // Function to handle a player move
    let handle_click = {
        let board_ot = board_ot.clone();
        let current_player = current_player.clone();
        let winner = winner.clone();
        Callback::from(move |(row, col): (usize, usize)| {
            let mut board = (*board_ot).clone();
            if board[row][col] == Tile::Empty {
                board[row][col] = if *current_player == 1 {
                    Tile::P1('T') // For Player 1, 'T'
                } else {
                    Tile::P2('O') // For Player 2, 'O'
                };
                board_ot.set(board);
                // Switch player turn
                current_player.set(if *current_player == 1 { 2 } else { 1 });
            }
        })
    };
    html! {
        <div>
            <h1>{ "Toot-Otto Game" }</h1>
            <div style="display: grid; grid-template-columns: repeat(6, 50px); gap: 5px;">
                { for board_ot.iter().enumerate().map(|(row, line)| html! {
                    { for line.iter().enumerate().map(|(col, &tile)| {
                        let on_click = {
                            let row = row;
                            let col = col;
                            handle_click.reform(move |_| (row, col))
                        };

                        html! {
                            <div style="border: 1px solid black; text-align: center; line-height: 50px; width: 50px; height: 50px;" onclick={on_click}>
                                { match tile {
                                    Tile::Empty => ' ',
                                    Tile::P1(letter) | Tile::P2(letter) => letter,
                                }}
                            </div>
                        }
                    })}
                })}
            </div>
            // Add logic to display current player or win conditions
        </div>
    }
}

fn main() {
    yew::Renderer::<Game>::new().render();
}
