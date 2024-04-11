use yew::prelude::*;
use yew_router::prelude::*;
mod connect4;
use connect4::{Board, Player, State};

mod toot_otto;
use toot_otto::{Board as TootBoard, Player as TootPlayer, State as TootState, Piece};

use std::io::{self, Write};


#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/game")]
    Game,
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
                <Link<Route> to={Route::Game}>{ "Play Game" }</Link<Route>>
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

    html! {
        <>
        <Link<Route> to={Route::Home}>{ "Back to Home" }</Link<Route>>
            <h1>{ "Connect Four" }</h1>
            <div style={button_style.clone()}>
                {
                    (0..board.cols).map(|col| {
                        html! {
                            <button style="text-align: center;" onclick={on_column_click.reform(move |_| col)}>
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
                                { format!("{:?}", cell) }
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











