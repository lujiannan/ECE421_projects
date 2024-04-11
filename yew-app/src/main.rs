use yew::prelude::*;
mod connect4;
use connect4::{Board, Player, State};


#[function_component(ConnectFourGame)]
fn connect_four_game() -> Html {
    let board = use_state(|| Board::new(6, 7)); // Initialize the board

    let on_column_click = {
        let board = board.clone();
        Callback::from(move |col: usize| {
            let mut b = (*board).clone(); // Clone the current board state
            b.insert_disc(col).ok(); // Ignore errors for simplicity
            board.set(b); // Update the board state
        })
    };

    html! {
        <>
            <h1>{ "Connect Four" }</h1>
            <div>
                {
                    (0..board.cols).map(|col| { // Use `cols` from your board structure
                        html! {
                            <button onclick={on_column_click.reform(move |_| col)}>
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







#[function_component(App)]
fn app() -> Html {
    html! {
        <ConnectFourGame />
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
