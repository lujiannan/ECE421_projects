use yew::prelude::*;
// use yew::events::InputData;
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

#[derive(Debug, PartialEq, Clone, Copy)]
enum Difficulty {
    Easy,
    Hard,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum PlayerIcon {
    Option1,
    Option2,
}
const ARMOR_IMG_URL: &str = "https://raw.githubusercontent.com/kooner27/421_projects/main/yew-app/static/armor.png";
const SWORD_IMG_URL: &str = "https://raw.githubusercontent.com/kooner27/421_projects/main/yew-app/static/sword.png";

#[derive(Debug, PartialEq, Clone, Copy)]
enum CompIcon {
    Option3,
    Option4,
}
const GEM_IMG_URL: &str = "https://raw.githubusercontent.com/kooner27/421_projects/main/yew-app/static/gem.png";
const HEART_IMG_URL: &str = "https://raw.githubusercontent.com/kooner27/421_projects/main/yew-app/static/heart.png";

#[derive(Properties, Clone, PartialEq)]
struct AppState {
    difficulty: Difficulty,
    player_icon: PlayerIcon,
    comp_icon: CompIcon,
}
impl AppState {
    fn new() -> Self {
        Self {
            difficulty: Difficulty::Easy,
            player_icon: PlayerIcon::Option1,
            comp_icon: CompIcon::Option3,
        }
    }
}

use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};

lazy_static! {
    static ref APP_STATE: Arc<Mutex<AppState>> = Arc::new(Mutex::new(AppState::new()));
}

#[function_component(Home)]
fn home() -> Html {

    let app_state = APP_STATE.lock().unwrap();

    let on_difficulty_change = {
        let app_state = Arc::clone(&APP_STATE);
        Callback::from(move |_| {
            let mut app_state = app_state.lock().unwrap();
            let value = if app_state.difficulty == Difficulty::Easy {
                Difficulty::Hard
            } else {
                Difficulty::Easy
            };
            app_state.difficulty = value;
        })
    };

    let on_player_icon_change = {
        let app_state = Arc::clone(&APP_STATE);
        Callback::from(move |_| {
            let mut app_state = app_state.lock().unwrap();
            let value = if app_state.player_icon == PlayerIcon::Option1 {
                PlayerIcon::Option2
            } else {
                PlayerIcon::Option1
            };
            app_state.player_icon = value;
        })
    };
    
    let on_comp_icon_change = {
        let app_state = Arc::clone(&APP_STATE);
        Callback::from(move |_| {
            let mut app_state = app_state.lock().unwrap();
            let value = if app_state.comp_icon == CompIcon::Option3 {
                CompIcon::Option4
            } else {
                CompIcon::Option3
            };
            app_state.comp_icon = value;
        })
    };


    html! {
        <div>
            <h1>{ "Welcome to our game center!" }</h1>
            <p>{ "We have simple implementations of Connect 4 and Toot-Otto, using Yew, WASM, Rust." }</p>
            <div>
                <p>{ "Select the difficulty of computer opponent:" }</p>     
                <input type="radio" id="easy" name="difficulty" value="easy" onclick={on_difficulty_change.clone()} checked={app_state.difficulty == Difficulty::Easy} />
                <label for="easy">{"Easy"}</label>
                <input type="radio" id="hard" name="difficulty" value="hard" onclick={on_difficulty_change.clone()} checked={app_state.difficulty == Difficulty::Hard}/>
                <label for="hard">{"Hard"}</label>
            </div>
            <div>
                <p>{ "Select the player icon you want:" }</p>     
                <input type="radio" id="option1" name="player_icon" value="option1" onclick={on_player_icon_change.clone()} checked={app_state.player_icon == PlayerIcon::Option1} />
                <label for="option1">{"option1"}</label>
                <img src={ARMOR_IMG_URL} width="80" height="80" />
                <input type="radio" id="option2" name="player_icon" value="option2" onclick={on_player_icon_change} checked={app_state.player_icon == PlayerIcon::Option2} />
                <label for="option2">{"option2"}</label>
                <img src={SWORD_IMG_URL} width="80" height="80" />
            </div>
            <div>
                <p>{ "Select the computer icon you want:" }</p>     
                <input type="radio" id="option3" name="comp_icon" value="option3" onclick={on_comp_icon_change.clone()} checked={app_state.comp_icon == CompIcon::Option3} />
                <label for="option3">{"option3"}</label>
                <img src={GEM_IMG_URL} width="80" height="80" />
                <input type="radio" id="option4" name="comp_icon" value="option4" onclick={on_comp_icon_change} checked={app_state.comp_icon == CompIcon::Option4} />
                <label for="option4">{"option4"}</label>
                <img src={HEART_IMG_URL} width="80" height="80" />
            </div>
            <nav>
                <p>{ "Instructions for each game are below." }</p>
                <Link<Route> to={Route::Instructions}>{ "Instructions" }</Link<Route>>
                <p>{ "Or start playing Connect 4 below." }</p>
                <Link<Route> to={Route::Game}>{ "Play Connect 4" }</Link<Route>>
                <p>{ "Or start playing Toot-Otto below." }</p>
                <Link<Route> to={Route::TootOttoGame}>{ "Play Toot-Otto" }</Link<Route>>
                
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

            <p>{ "Connect Four is a two-player connection game in which the players take turns dropping colored discs from the top into a seven-column, six-row vertically suspended grid. The objective of the game is to be the first to form a horizontal, vertical, or diagonal line of four of one's own discs." }</p>
            <br/>
            <div><h5>{ "To play Connect 4 follow the following steps:" }</h5></div>
            <ul>
                <li>{ "A new game describes discs of which color belongs to which player" }</li>
                <li>{ "Click on the desired column on the game board to place your disc" }</li>
                <li>{ "Try to connect 4 of your colored discs either horizontally or vertically or diagonally" }</li>
            </ul>
            <br/>
            { "For More information on Connect 4 click " }
            <a href="https://en.wikipedia.org/wiki/Connect_Four">{ "here" }</a>
            <br/>
            
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

    let app_state_borrowed = APP_STATE.lock().unwrap();

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
            <h2>{ format!("App State: Difficulty - {:?}, Player Icon - {:?}, Comp Icon - {:?}", 
            app_state_borrowed.difficulty, app_state_borrowed.player_icon, app_state_borrowed.comp_icon) }</h2>
            
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
                        // html! {
                        //     <div style="border: 1px solid black; text-align: center; line-height: 100px;">
                        //         // { format!("{:?}", cell) }
                        //         { display_cell(cell) }
                        //     </div>
                        // }
                        html! {
                            <div style="border: 1px solid black; text-align: center; line-height: 100px;">
                            {
                                match cell {
                                    connect4::Cell::Empty => html! { },
                                    connect4::Cell::Occupied(Player::Red) => html! { <img src={ARMOR_IMG_URL} width="80" height="80" /> },
                                    connect4::Cell::Occupied(Player::Yellow) => html! { <img src={SWORD_IMG_URL} width="80" height="80" /> },
                                }
                            }
                            </div>
                        }
                    })
                }
            </div>
            <div>
                {
                    match board.state {
                        State::Won(player) => html! { <p>{ format!("Player {:?} wins! Refresh to reset game.", player) }</p> },
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









