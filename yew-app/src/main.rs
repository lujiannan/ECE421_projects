use yew::prelude::*;
// use yew::events::InputData;
use yew_router::prelude::*;
mod connect4;
use connect4::{Board, Cell, Player, State};

mod toot_otto;
use toot_otto::{
    Board as TootBoard, Cell as TootCell, Piece, Player as TootPlayer, State as TootState,
};

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
    None,
    Easy,
    Hard,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum PlayerIcon {
    Option1,
    Option2,
}
const ARMOR_IMG_URL: &str =
    "https://raw.githubusercontent.com/kooner27/421_projects/main/yew-app/static/armor.png";
const SWORD_IMG_URL: &str =
    "https://raw.githubusercontent.com/kooner27/421_projects/main/yew-app/static/sword.png";

#[derive(Debug, PartialEq, Clone, Copy)]
enum CompIcon {
    Option3,
    Option4,
}
const GEM_IMG_URL: &str =
    "https://raw.githubusercontent.com/kooner27/421_projects/main/yew-app/static/gem.png";
const HEART_IMG_URL: &str =
    "https://raw.githubusercontent.com/kooner27/421_projects/main/yew-app/static/heart.png";

#[derive(Properties, Clone, PartialEq)]
struct AppState {
    difficulty: Difficulty,
    player_icon: PlayerIcon,
    comp_icon: CompIcon,
}
impl AppState {
    fn new() -> Self {
        Self {
            difficulty: Difficulty::None,
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

            <p>{ "Select an icon for player1:" }</p>
            <div style="display: flex; align-items: center;">
                <input type="radio" id="option1" name="player_icon" value="option1" onclick={on_player_icon_change.clone()} checked={app_state.player_icon == PlayerIcon::Option1} />
                <label for="option1">{"option1"}</label>
                <img src={ARMOR_IMG_URL} width="60" height="60" />
                <input type="radio" id="option2" name="player_icon" value="option2" onclick={on_player_icon_change} checked={app_state.player_icon == PlayerIcon::Option2} />
                <label for="option2">{"option2"}</label>
                <img src={SWORD_IMG_URL} width="60" height="60" />
            </div>

            <p>{ "Select an icon for player2:" }</p>
            <div style="display: flex; align-items: center;">
                <input type="radio" id="option3" name="comp_icon" value="option3" onclick={on_comp_icon_change.clone()} checked={app_state.comp_icon == CompIcon::Option3} />
                <label for="option3">{"option3"}</label>
                <img src={GEM_IMG_URL} width="60" height="60" />
                <input type="radio" id="option4" name="comp_icon" value="option4" onclick={on_comp_icon_change} checked={app_state.comp_icon == CompIcon::Option4} />
                <label for="option4">{"option4"}</label>
                <img src={HEART_IMG_URL} width="60" height="60" />
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
    let players_icon = match app_state_borrowed.player_icon {
        PlayerIcon::Option1 => ARMOR_IMG_URL,
        PlayerIcon::Option2 => SWORD_IMG_URL,
    };
    let comp_icon = match app_state_borrowed.comp_icon {
        CompIcon::Option3 => GEM_IMG_URL,
        CompIcon::Option4 => HEART_IMG_URL,
    };

    let on_difficulty_change = {
        let app_state = Arc::clone(&APP_STATE);
        Callback::from(move |d: &str| {
            let mut app_state = app_state.lock().unwrap();
            app_state.difficulty = match d {
                "none" => Difficulty::None,
                "easy" => Difficulty::Easy,
                "hard" => Difficulty::Hard,
                _ => Difficulty::None,
            };
        })
    };

    let board = use_state(|| Board::new(6, 7)); // Initialize the board
    let hovered_col: UseStateHandle<Option<usize>> = use_state(|| None);

    let current_player = match board.current_turn {
        Player::Red => "Player1",
        Player::Yellow => "Player2",
    };

    let current_player_icon = match board.current_turn {
        Player::Red => players_icon,
        Player::Yellow => comp_icon,
    };

    let player1_done = use_state(|| false);
    let on_column_click_comp = {
        let board = board.clone();
        let app_state_borrowed = app_state_borrowed.clone();
        let player1_clicked = player1_done.clone();
        Callback::from(move |_col: usize| {
            let mut b = (*board).clone();
            // if the current status is player vs. computer
            match app_state_borrowed.difficulty {
                Difficulty::None => (),
                Difficulty::Easy => {
                    if let Err(e) = b.computer_move() {
                        println!("Error: {}", e);
                    } else {
                        board.set(b);
                        player1_clicked.set(false);
                    }
                }
                Difficulty::Hard => {
                    if let Err(e) = b.computer_move_hard(_col) {
                        println!("Error: {}", e);
                    } else {
                        board.set(b);
                        player1_clicked.set(false);
                    }
                }
            }
        })
    };

    let on_column_click = {
        let board = board.clone();
        let hovered_col = hovered_col.clone();
        let player1_done = player1_done.clone();
        Callback::from(move |col: usize| {
            let mut b = (*board).clone(); // Clone the current board state
            b.insert_disc(col).ok(); // Ignore errors for simplicity
            if b.state != connect4::State::Running {
                hovered_col.set(None);
            }
            player1_done.set(true);
            board.set(b); // Update the board state
        })
    };

    let handle_mouseover = {
        let hovered_col = hovered_col.clone();
        Callback::from(move |col: usize| {
            hovered_col.set(Some(col));
        })
    };
    let handle_mouseout = {
        let hovered_col = hovered_col.clone();
        Callback::from(move |_col: usize| {
            hovered_col.set(None);
        })
    };

    let pixel_size = "100px";
    let grid_style = format!(
        "display: grid; grid-template-columns: repeat({}, {}); grid-auto-rows: {};",
        board.cols, pixel_size, pixel_size
    );

    let cell_style_locked = "
    border: 1px solid black;
    text-align: center;
    line-height: 100px;
    ";
    let cell_style_hovered = "
    border: 1px solid black;
    text-align: center;
    line-height: 100px;
    background-color: lightgray;
    box-shadow: 2px 2px 5px rgba(0, 0, 0, 0.3);
    ";

    html! {
        <>
        <Link<Route> to={Route::Home}>{ "Back to Home" }</Link<Route>>
            <h1>{ "Connect Four" }</h1>

            <h2 style="display: flex; align-items: center;">
                { format!("Status: ") }
                { format!("Player1 - ") }
                <img src={players_icon} width="50" height="50" />
                { format!(", Player2 -") }
                <img src={comp_icon} width="50" height="50" />
            </h2>
            <div style="display: flex; align-items: center;">
                <text>{ "AI: "}</text>
                <input type="radio" id="none" name="difficulty" value="none" onclick={on_difficulty_change.reform(move |_| "none")} checked={app_state_borrowed.difficulty == Difficulty::None} />
                <label for="none">{"None"}</label>
                <input type="radio" id="easy" name="difficulty" value="easy" onclick={on_difficulty_change.reform(move |_| "easy")} checked={app_state_borrowed.difficulty == Difficulty::Easy} />
                <label for="easy">{"Easy"}</label>
                <input type="radio" id="hard" name="difficulty" value="hard" onclick={on_difficulty_change.reform(move |_| "hard")} checked={app_state_borrowed.difficulty == Difficulty::Hard}/>
                <label for="hard">{"Hard"}</label>
            </div>
            <p style="display: flex; align-items: center;">
                { format!("Current turn: ") }
                <img src={current_player_icon} width="50" height="50" />
                { format!(" ({})", current_player) }
            </p>

            <div style={grid_style.clone()}>
                {
                    for board.grid.iter().enumerate().map(|(_row, line)| {
                        html! {
                            {
                                for line.iter().enumerate().map(|(col, &cell)| {
                                    let mut cell_style: &str = "";
                                    if let Some(hovered_col) = *hovered_col {
                                        if hovered_col == col {
                                            cell_style = cell_style_hovered;
                                            // UI prediction of the future position of next move
                                            
                                        } else {
                                            cell_style = cell_style_locked;
                                        }
                                    } else {
                                        cell_style = cell_style_locked;
                                    };
                                    let is_enabled = matches!(board.state, connect4::State::Running);
                                    // robot move
                                    if *player1_done == true {
                                        on_column_click_comp.emit(col);
                                    }
                                    html! {
                                        <button
                                        style={cell_style}
                                        onmouseenter={
                                            if is_enabled {
                                                handle_mouseover.reform(move |_| col)
                                            } else {
                                                handle_mouseout.reform(move |_| col)
                                            }
                                        }
                                        onmouseleave={handle_mouseout.reform(move |_| col)}
                                        onclick={on_column_click.reform(move |_| col)}
                                        disabled={!is_enabled}
                                        >
                                            {
                                                match cell {
                                                    connect4::Cell::Empty => html! {},
                                                    connect4::Cell::Occupied(Player::Red) => html! { <img src={players_icon} width="80" height="80" /> },
                                                    connect4::Cell::Occupied(Player::Yellow) => html! { <img src={comp_icon} width="80" height="80" /> },
                                                }
                                            }
                                        </button>
                                    }
                                })
                            }
                        }
                    })
                }
            </div>
            <div>
                {
                    match board.state {
                        State::Won(player) => html! {
                            <p>
                                {
                                    format!("{} wins! Refresh to reset game.", if player == Player::Red {"Player1"} else {"Player2"})
                                }
                            </p>
                        },
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

    let app_state_borrowed = APP_STATE.lock().unwrap();
    let on_difficulty_change = {
        let app_state = Arc::clone(&APP_STATE);
        Callback::from(move |d: &str| {
            let mut app_state = app_state.lock().unwrap();
            app_state.difficulty = match d {
                "none" => Difficulty::None,
                "easy" => Difficulty::Easy,
                "hard" => Difficulty::Hard,
                _ => Difficulty::None,
            };
        })
    };

    let hovered_col: UseStateHandle<Option<usize>> = use_state(|| None);

    let player1_done = use_state(|| false);
    let on_column_click_comp = {
        let board = board.clone();
        let app_state_borrowed = app_state_borrowed.clone();
        let player1_done = player1_done.clone();
        Callback::from(move |_col: usize| {
            let mut b = (*board).clone();
            // if the current status is player vs. computer
            if app_state_borrowed.difficulty != Difficulty::None {
                if let Err(e) = b.computer_move() {
                    println!("Error: {}", e);
                } else {
                    board.set(b);
                    player1_done.set(false);
                }
            }
        })
    };

    let on_column_click = {
        let board = board.clone();
        let selected_piece = selected_piece.clone();
        let hovered_col = hovered_col.clone();
        let player1_done = player1_done.clone();
        Callback::from(move |col: usize| {
            if let Some(piece) = *selected_piece {
                let mut b = (*board).clone();
                hovered_col.set(None);
                b.insert_piece(col, piece).ok(); // Handling the insertion and ignoring errors
                board.set(b); // Update the board state
                player1_done.set(true);
                selected_piece.set(None); // Reset the selected piece after placing it
            }
        })
    };

    let handle_mouseover = {
        let selected_piece = selected_piece.clone();
        let is_selected = if *selected_piece == None { false } else { true };
        let hovered_col = hovered_col.clone();
        Callback::from(move |col: usize| {
            if is_selected {
                hovered_col.set(Some(col));
            } else {
                hovered_col.set(None);
            }
        })
    };
    let handle_mouseout = {
        let hovered_col = hovered_col.clone();
        Callback::from(move |_col: usize| {
            hovered_col.set(None);
        })
    };

    let pixel_size = "80px"; // Smaller pieces for a more complex board
    let grid_style = format!(
        "display: grid; grid-template-columns: repeat({}, {}); grid-auto-rows: {};",
        board.cols, pixel_size, pixel_size
    );
    let current_player = match board.current_turn {
        TootPlayer::Toot => "TOOT",
        TootPlayer::Otto => "OTTO",
    };

    let cell_style_locked = "
    border: 1px solid black;
    text-align: center;
    line-height: 100px;
    ";
    let cell_style_hovered = "
    border: 1px solid black;
    text-align: center;
    line-height: 100px;
    background-color: lightgray;
    box-shadow: 2px 2px 5px rgba(0, 0, 0, 0.3);
    ";
    let btn_style_regular: &str = "color: dimgray; margin: 4px;";
    let btn_style_selected = "background-color: lightgray; color: black; margin: 4px;";

    html! {
        <>
            <Link<Route> to={Route::Home}>{ "Back to Home" }</Link<Route>>
            <h1>{ "TOOT-OTTO" }</h1>
            <div style="display: flex; align-items: center;">
                <text>{ "AI: "}</text>
                <input type="radio" id="none" name="difficulty" value="none" onclick={on_difficulty_change.reform(move |_| "none")} checked={app_state_borrowed.difficulty == Difficulty::None} />
                <label for="none">{"None"}</label>
                <input type="radio" id="easy" name="difficulty" value="easy" onclick={on_difficulty_change.reform(move |_| "easy")} checked={app_state_borrowed.difficulty == Difficulty::Easy} />
                <label for="easy">{"Easy"}</label>
                <input type="radio" id="hard" name="difficulty" value="hard" onclick={on_difficulty_change.reform(move |_| "hard")} checked={app_state_borrowed.difficulty == Difficulty::Hard}/>
                <label for="hard">{"Hard"}</label>
            </div>
            <p>{ format!("Current turn: {}", current_player) }</p>
            <div>
                <button
                    style={
                        if *selected_piece == Some(Piece::T) {btn_style_selected}
                        else {btn_style_regular}
                    }
                    onclick={on_piece_select.reform(|_| Piece::T)}
                    disabled={!matches!(board.state, toot_otto::State::Running)}
                >
                    { "Select T" }
                </button>
                <button
                    style={
                        if *selected_piece == Some(Piece::O) {btn_style_selected}
                        else {btn_style_regular}
                    }
                    onclick={on_piece_select.reform(|_| Piece::O)}
                    disabled={!matches!(board.state, toot_otto::State::Running)}
                >
                    { "Select O" }
                </button>
            </div>
            <div style={grid_style.clone()}>
                {
                    for board.grid.iter().enumerate().map(|(_row, line)| {
                        html! {
                            {
                                for line.iter().enumerate().map(|(col, &cell)| {
                                    let mut cell_style: &str = "";
                                    if let Some(hovered_col) = *hovered_col {
                                        if hovered_col == col {
                                            cell_style = cell_style_hovered;
                                        } else {
                                            cell_style = cell_style_locked;
                                        }
                                    } else {
                                        cell_style = cell_style_locked;
                                    };
                                    // robot move
                                    if *player1_done == true {
                                        on_column_click_comp.emit(col);
                                    }
                                    html! {
                                        <button
                                        style={cell_style}
                                        onmouseenter={
                                            handle_mouseover.reform(move |_| col)
                                        }
                                        onmouseleave={handle_mouseout.reform(move |_| col)}
                                        onclick={on_column_click.reform(move |_| col)}
                                        >
                                            {display_toot_piece(&cell)}
                                        </button>
                                    }
                                })
                            }
                        }
                    })
                }
            </div>
            <div>
                {
                    match board.state {
                        TootState::Won(player) => html! { <p>{ format!("Player {:?} wins! Refresh to reset game.", player) }</p> },
                        TootState::Draw => html! { <p>{ "The game is a draw!" }</p> },
                        TootState::Running => html! { <p>{ "Game is in progress..." }</p> },
                    }
                }
            </div>
        </>
    }
}
