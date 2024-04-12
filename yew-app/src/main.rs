use yew::prelude::*;
use yew_router::prelude::*;
mod connect4;
use connect4::{Board, Player, State, Cell};

mod toot_otto;
use toot_otto::{Board as TootBoard, Player as TootPlayer, State as TootState, Piece, Cell as TootCell};

use std::io::{self, Write};


#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/game")]
    Game,
    #[at("/toot-otto-game")]
    TootOttoGame,
    #[at("/instructions")]
    Instructions,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[function_component(Home)]
fn home() -> Html {
    html! {
        <div>
            <h1>{ "Welcome to Connect Four!" }</h1>
            <nav>
                <Link<Route> to={Route::Game}>{ "Play Connect 4" }</Link<Route>>
                <Link<Route> to={Route::TootOttoGame}>{ "Play Toot-Otto" }</Link<Route>>
                <Link<Route> to={Route::Instructions}>{ "Instructions" }</Link<Route>>
            </nav>
        </div>
    }
}

#[function_component(Instructions)]
fn instructions() -> Html {
    html! {
        <div>
            <h1>{ "Instructions" }</h1>
            <p>{ "This page will provide instructions on how to play Connect Four." }</p>
            <p>{ "Place your discs by clicking the buttons and try to get four in a row." }</p>
            <Link<Route> to={Route::Home}>{ "Back to Home" }</Link<Route>>
        </div>
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            // <Switch<Route> render={Switch::render(switch)} />
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::Game => html! { <ConnectFourGame /> },
        Route::TootOttoGame => html! { <TootOttoGame /> },
        Route::Instructions => html! { <Instructions /> },
        Route::NotFound => html! { <h1>{ "404 Not Found" }</h1> },
    }
}

// #[function_component(App)]
// fn app() -> Html {
//     html! {
//         <ConnectFourGame />
//     }
// }

fn main() {
    yew::Renderer::<App>::new().render();
}

fn display_cell(cell: &Cell) -> String {
    match cell {
        Cell::Empty => " ".to_string(),
        Cell::Occupied(player) => match player {
            Player::Red => "Red".to_string(),
            Player::Yellow => "Yellow".to_string(),
        },
    }
}


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
    let pixel_size = "100px";
    let button_style = format!("display: grid; grid-template-columns: repeat({}, {});", board.cols, pixel_size);
    let grid_style = format!("display: grid; grid-template-columns: repeat({}, {}); grid-auto-rows: {};", board.cols, pixel_size, pixel_size);
    
    let button_style_active = "text-align: center; background-color: initial;";
    let button_style_greyed = "text-align: center; background-color: grey;";

    html! {
        <>
        <Link<Route> to={Route::Home}>{ "Back to Home" }</Link<Route>>
            <h1>{ "Connect Four" }</h1>
            <div style={button_style.clone()}>
            {
                (0..board.cols).map(|col| {
                    let is_disabled = matches!(board.state, State::Running);
                    let button_style = if is_disabled { button_style_active } else { button_style_greyed };
                    html! {
                        <button style={button_style} onclick={on_column_click.reform(move |_| col)} disabled={!is_disabled}>
                            { format!("Drop in Col {}", col) }
                        </button>
                    }
                }).collect::<Html>()
            }
            </div>
            <div style={grid_style.clone()}>
                {
                    for board.grid.iter().flatten().map(|cell| {
                        html! {
                            <div style="border: 1px solid black; text-align: center; line-height: 100px;">
                                // { format!("{:?}", cell) }
                                { display_cell(cell) }
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






fn display_toot_piece(cell: &TootCell) -> String {
    match cell {
        TootCell::Empty => " ".to_string(),
        TootCell::Occupied(piece) => match piece {
            Piece::T => "T".to_string(),
            Piece::O => "O".to_string(),
        },
    }
}



////////////////// toot otto
#[function_component(TootOttoGame)]
fn toot_otto_game() -> Html {
    let board = use_state(|| TootBoard::new(4, 6)); // Standard TOOT-OTTO board size

    // State to keep track of the currently selected piece
    let selected_piece = use_state(|| None);

    let on_piece_select = {
        let selected_piece = selected_piece.clone();
        Callback::from(move |piece: Piece| {
            selected_piece.set(Some(piece));
        })
    };

    let on_column_click = {
        let board = board.clone();
        let selected_piece = selected_piece.clone();
        Callback::from(move |col: usize| {
            if let Some(piece) = *selected_piece {
                let mut b = (*board).clone();
                b.insert_piece(col, piece).ok(); // Handling the insertion and ignoring errors
                board.set(b); // Update the board state
                selected_piece.set(None); // Reset the selected piece after placing it
            }
        })
    };

    let pixel_size = "80px"; // Smaller pieces for a more complex board
    let grid_style = format!("display: grid; grid-template-columns: repeat({}, {}); grid-auto-rows: {};", board.cols, pixel_size, pixel_size);
    let current_player = match board.current_turn {
        TootPlayer::Toot => "TOOT",
        TootPlayer::Otto => "OTTO",
    };

    html! {
        <>
            <Link<Route> to={Route::Home}>{ "Back to Home" }</Link<Route>>
            <h1>{ "TOOT-OTTO" }</h1>
            <p>{ format!("Current turn for: {}", current_player) }</p>
            <div>
                <button onclick={on_piece_select.reform(|_| Piece::T)}>{ "Select T" }</button>
                <button onclick={on_piece_select.reform(|_| Piece::O)}>{ "Select O" }</button>
            </div>
            <div style={grid_style.clone()}>
                {
                    for board.grid.iter().flatten().map(|cell| {
                        html! {
                            <div style="border: 1px solid black; text-align: center; line-height: 80px;">
                            // { format!("{:?}", cell) }
                            { display_toot_piece(cell) }
                            </div>
                        }
                    })
                }
            </div>
            <div>
                {
                    (0..board.cols).map(|col| {
                        html! {
                            <button onclick={on_column_click.reform(move |_| col)} disabled={selected_piece.is_none()}>
                                { format!("Place in Col {}", col) }
                            </button>
                        }
                    }).collect::<Html>()
                }
            </div>
            <div>
                {
                    match board.state {
                        TootState::Won(player) => html! { <p>{ format!("Player {:?} wins!", player) }</p> },
                        TootState::Draw => html! { <p>{ "The game is a draw!" }</p> },
                        TootState::Running => html! { <p>{ "Game is in progress..." }</p> },
                    }
                }
            </div>
        </>
    }
}









