use yew::prelude::*;
// use yew::events::InputData;
use yew_router::prelude::*;
mod connect4;
use connect4::{Board, Cell, Player, State};

mod toot_otto;
use toot_otto::{
    Board as TootBoard, Cell as TootCell, Piece, Player as TootPlayer, State as TootState,
};

use serde::{Deserialize, Serialize};
use serde_json::*;
use std::io::{self, Write};
use web_sys::window;

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

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
enum Difficulty {
    None,
    Easy,
    Hard,
}

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
enum PlayerIcon {
    Option1,
    Option2,
}
const ARMOR_IMG_URL: &str =
    "https://raw.githubusercontent.com/kooner27/421_projects/main/yew-app/static/armor.png";
const SWORD_IMG_URL: &str =
    "https://raw.githubusercontent.com/kooner27/421_projects/main/yew-app/static/sword.png";

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
enum CompIcon {
    Option3,
    Option4,
}
const GEM_IMG_URL: &str =
    "https://raw.githubusercontent.com/kooner27/421_projects/main/yew-app/static/gem.png";
const HEART_IMG_URL: &str =
    "https://raw.githubusercontent.com/kooner27/421_projects/main/yew-app/static/heart.png";

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
enum PlayerAsTootOtto {
    PlayerToot,
    PlayerOtto,
}

#[derive(Properties, Clone, PartialEq, Serialize, Deserialize)]
struct AppState {
    difficulty: Difficulty,
    player_icon: PlayerIcon,
    comp_icon: CompIcon,
    player_as_toot_otto: PlayerAsTootOtto,
}
impl AppState {
    fn new() -> Self {
        Self {
            difficulty: Difficulty::None,
            player_icon: PlayerIcon::Option1,
            comp_icon: CompIcon::Option3,
            player_as_toot_otto: PlayerAsTootOtto::PlayerToot,
        }
    }
}

use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};

lazy_static! {
    static ref APP_STATE: Arc<Mutex<AppState>> = Arc::new(Mutex::new(AppState::new()));
}

fn save_state(state: &AppState) {
    let window = window().expect("no global `window` exists");
    let storage = window.local_storage().unwrap().unwrap();
    storage
        .set_item("appState", &serde_json::to_string(state).unwrap())
        .unwrap();
}

fn load_state() -> Option<AppState> {
    let window = window().expect("no global `window` exists");
    let storage = window.local_storage().unwrap().unwrap();
    let state_json = storage.get_item("appState").unwrap()?;
    Some(serde_json::from_str(&state_json).unwrap())
}
//alternative version to return state, incase serialized data returned is not compatible with AppState
// fn load_state() -> Result<AppState, serde_json::Error> {
//     let window = window().expect("no global `window` exists");
//     let storage = window.local_storage().unwrap().unwrap();
//     let state_json = storage.get_item("appState").unwrap()?;
//     serde_json::from_str(&state_json)
// }

#[function_component(Home)]
fn home() -> Html {
    let mut app_state = APP_STATE.lock().unwrap();

    // Load state when the application starts
    let loaded_state = load_state();
    if let Some(state) = loaded_state {
        *app_state = state;
    }
    // // Load state when the application starts
    // match load_state() {
    //     Ok(state) => *app_state = state,
    //     Err(e) => {
    //         // Handle the error here, e.g. log it or show an error message
    //         console::log_1(&format!("Failed to load state: {}", e).into());
    //     }
    // }

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

            // Save state when the player icon changes
            save_state(&app_state);
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

            // Save state when the computer icon changes
            save_state(&app_state);
        })
    };

    // let on_player_select_toototto = {
    //     let app_state = Arc::clone(&APP_STATE);
    //     Callback::from(move |_| {
    //         let mut app_state = app_state.lock().unwrap();
    //         let value = if app_state.player_as_toot_otto == PlayerAsTootOtto::PlayerToot {
    //             PlayerAsTootOtto::PlayerOtto
    //         } else {
    //             PlayerAsTootOtto::PlayerToot
    //         };
    //         app_state.player_as_toot_otto = value;

    //         // Save state when the computer icon changes
    //         save_state(&app_state);
    //     })
    // };

    // let on_difficulty_change = {
    //     let app_state = Arc::clone(&APP_STATE);
    //     Callback::from(move |value: String| {
    //         let mut app_state = app_state.lock().unwrap();
    //         app_state.difficulty = match value.as_str() {
    //             "easy" => Difficulty::Easy,
    //             "hard" => Difficulty::Hard,
    //             _ => Difficulty::None,
    //         };
    //         // Save state when the difficulty changes
    //         save_state(&app_state);
    //     })
    // };

    html! {
        <div>
            <h1 class="text_titles">{ "Welcome to our game center!" }</h1>
            <p>{ "We have simple implementations of Connect Four, and, Toot and Otto, using Yew, WASM, and Rust." }</p>
            <nav>
                <text>{ "Instructions for each game:  " }</text>
                <Link<Route> to={Route::Instructions}>{ "Instructions" }</Link<Route>>
            </nav>

            <h1 class="text_titles">{ "I want to play Connect Four..." }</h1>
            <p>{ "Select an icon for Player1:" }</p>
            <div class="radio-buttons" style="display: flex; align-items: center;">
                <img src={ARMOR_IMG_URL} width="80" height="80" />
                <label class="l-radio" for="option1">
                    <input type="radio" id="option1" name="player_icon" value="option1" onclick={on_player_icon_change.clone()} checked={app_state.player_icon == PlayerIcon::Option1} />
                    <span>{"Armor"}</span>
                </label>
                
                <label class="l-radio" for="option2">
                    <input type="radio" id="option2" name="player_icon" value="option2" onclick={on_player_icon_change} checked={app_state.player_icon == PlayerIcon::Option2} />
                    <span>{"Spear"}</span>
                </label>
                <img src={SWORD_IMG_URL} width="70" height="70" />
            </div>

            <p>{ "Select an icon for Player2/Computer:" }</p>
            <div class="radio-buttons" style="display: flex; align-items: center;">
                <img src={GEM_IMG_URL} width="90" height="90" />
                <label class="l-radio" for="option3">
                    <input type="radio" id="option3" name="comp_icon" value="option3" onclick={on_comp_icon_change.clone()} checked={app_state.comp_icon == CompIcon::Option3} />
                    <span>{"Gem"}</span>
                </label>
                
                <label class="l-radio" for="option4">
                    <input type="radio" id="option4" name="comp_icon" value="option4" onclick={on_comp_icon_change} checked={app_state.comp_icon == CompIcon::Option4} />
                    <span>{"Heart"}</span>
                </label>
                <img src={HEART_IMG_URL} width="80" height="80" />
            </div>

            // <p>{ "Select the play mode ('none' for 2-human players, 'easy' for easy computer opponent, 'hard' for hard computer opponent):" }</p>
            // <div style="display: flex; align-items: center;">
            //     <text>{ "AI: "}</text>
            //     <input type="radio" id="none" name="difficulty" value="none" onclick={on_difficulty_change.reform(|_| "none".to_string())} checked={app_state.difficulty == Difficulty::None} />
            //     <label for="none">{"None"}</label>
            //     <input type="radio" id="easy" name="difficulty" value="easy" onclick={on_difficulty_change.reform(|_| "easy".to_string())} checked={app_state.difficulty == Difficulty::Easy} />
            //     <label for="easy">{"Easy"}</label>
            //     <input type="radio" id="hard" name="difficulty" value="hard" onclick={on_difficulty_change.reform(|_| "hard".to_string())} checked={app_state.difficulty == Difficulty::Hard}/>
            //     <label for="hard">{"Hard"}</label>
            // </div>

            <nav>
                <Link<Route> to={Route::Game}>
                    <button class="btn-forward">
                        <span class="circle" aria-hidden="true">
                            <span class="icon arrow"></span>
                        </span>
                        <span class="button-text">{"Play Connect Four"}</span>
                    </button>
                </Link<Route>>
            </nav>

            <h1 class="text_titles">{ "I want to play TooT and Otto..." }</h1>
            <p>{ "Player 1 is TOOT and always starts first, the Player2/Computer goes after." }</p>
            // <p>{ "Select player1's word (and player2/computer will be other word):" }</p>
            // <div class="radio-buttons" style="display: flex; align-items: center;">
            //     <label class="l-radio" for="TOOT">
            //         <input type="radio" id="TOOT" name="player1" value="TOOT" onclick={on_player_select_toototto.clone()} checked={app_state.player_as_toot_otto == PlayerAsTootOtto::PlayerToot} />
            //         <span>{"TOOT"}</span>
            //     </label>
            //     <label class="l-radio" for="OTTO">
            //         <input type="radio" id="OTTO" name="player1" value="OTTO" onclick={on_player_select_toototto} checked={app_state.player_as_toot_otto == PlayerAsTootOtto::PlayerOtto} />
            //         <span>{"OTTO"}</span>
            //     </label>
            // </div>

            // <p>{ "Select the play mode ('none' for 2-human players, 'easy' for easy computer opponent, 'hard' for hard computer opponent):" }</p>
            // <div style="display: flex; align-items: center;">
            //     <text>{ "AI: "}</text>
            //     <input type="radio" id="none" name="difficulty" value="none" onclick={on_difficulty_change.reform(|_| "none".to_string())} checked={app_state.difficulty == Difficulty::None} />
            //     <label for="none">{"None"}</label>
            //     <input type="radio" id="easy" name="difficulty" value="easy" onclick={on_difficulty_change.reform(|_| "easy".to_string())} checked={app_state.difficulty == Difficulty::Easy} />
            //     <label for="easy">{"Easy"}</label>
            //     <input type="radio" id="hard" name="difficulty" value="hard" onclick={on_difficulty_change.reform(|_| "hard".to_string())} checked={app_state.difficulty == Difficulty::Hard}/>
            //     <label for="hard">{"Hard"}</label>
            // </div>
            <nav>
            <Link<Route> to={Route::TootOttoGame}><button class="btn-forward">
                <span class="circle" aria-hidden="true">
                        <span class="icon arrow"></span>
                    </span>
                    <span class="button-text">{"Play Toot and Otto"}</span>
                </button>
            </Link<Route>>
        </nav>
        </div>
    }
}

#[function_component(Instructions)]
fn instructions() -> Html {
    html! {
        <div class="content_padding">
            <Link<Route> to={Route::Home}>
                <button class="btn-back">
                    <span class="circle" aria-hidden="true">
                        <span class="icon arrow"></span>
                    </span>
                    <span class="button-text">{"Back to Home"}</span>
                </button>
            </Link<Route>>
            <h1 class="text_titles">{ "Instructions" }</h1>
            <h2 class="text_titles">{ "How to play our Connect Four:" }</h2>
            <p>{ "Connect Four is a two-player connection game in which the players take turns
            dropping their disc/icons from the top into a seven-column, six-row vertically suspended 
            grid. The objective of the game is to be the first to form a horizontal, vertical, 
            or diagonal line of four of one's own discs/icons. In our implementation, 
            a player can play against a person or a computer (with easy or hard modes); 
            there are different two icons, per player, to choose between as their representative icon of the game; 
            the Player1 goes first, so Player2/Computer is always second to play;
            a tie is made when the board fills without a winner." }</p>

            { "For more information on the official Connect Four game, click: " }
            <a href="https://en.wikipedia.org/wiki/Connect_Four">{ "here" }</a>
            <br/>

            <h2 class="text_titles">{ "How to play our Toot and Otto:" }</h2>
            <p>{ "Toot and Otto is a two-player connection game in which the players take turns
            dropping their selections of O or T (as representations of disc/icons) 
            from the top into a six-column, four-row vertically suspended 
            grid. Players choose if they will represent Toot or Otto before the game begins. 
            The objective of the game is to be the first to form a horizontal, vertical, 
            or diagonal spelling of one's own name (i.e. if they are Toot, they want to make TOOT somewhere in the grid). 
            In our implementation, 
            a player can play against a person or a computer (with easy or hard modes); 
            the Player1 goes first and is always TOOT (in the official game, TOOT always goes first); 
            so Player2/Computer is always OTTO;
            a tie is made when the board fills without a winner." }</p>
            { "For more information on the official Toot and Otto game, click: " }
            <a href="https://boardgamegeek.com/boardgame/19530/toot-and-otto">{ "here" }</a>
            <br/>
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

// fn display_cell(cell: &Cell) -> String {
//     match cell {
//         Cell::Empty => " ".to_string(),
//         Cell::Occupied(player) => match player {
//             Player::Red => "Red".to_string(),
//             Player::Yellow => "Yellow".to_string(),
//         },
//     }
// }

#[function_component(ConnectFourGame)]
fn connect_four_game() -> Html {
    let mut app_state_borrowed = APP_STATE.lock().unwrap();

    // Load state when the application starts
    let loaded_state = load_state();
    if let Some(state) = loaded_state {
        *app_state_borrowed = state;
    }

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

            // Save state when the difficulty changes
            save_state(&app_state);
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
                    if let Some((_row, col)) = b.last_move {
                        if let Err(e) = b.computer_move_hard(col) {
                            println!("Error: {}", e);
                        } else {
                            println!("Hard moved");
                            board.set(b);
                            player1_clicked.set(false);
                        }
                    } else {
                        if let Err(e) = b.computer_move_hard(_col) {
                            println!("Error: {}", e);
                        } else {
                            println!("Hard moved");
                            board.set(b);
                            player1_clicked.set(false);
                        }
                    }
                }
            }
        })
    };

    let predicted_pos: UseStateHandle<Option<(usize, usize)>> = use_state(|| None);

    let on_column_click = {
        let board = board.clone();
        let hovered_col = hovered_col.clone();
        let player1_done = player1_done.clone();
        let predicted_pos = predicted_pos.clone();
        Callback::from(move |col: usize| {
            let mut b = (*board).clone(); // Clone the current board state
            b.insert_disc(col).ok(); // Ignore errors for simplicity
            let b_cpy = b.clone();
            if b.state != connect4::State::Running {
                hovered_col.set(None);
                predicted_pos.set(None);
            }
            player1_done.set(true);
            board.set(b); // Update the board state
            predicted_pos.set(b_cpy.predict_disc(col));
        })
    };

    let handle_mouseover = {
        let board = board.clone();
        let hovered_col = hovered_col.clone();
        let predicted_pos = predicted_pos.clone();
        Callback::from(move |col: usize| {
            let b = (*board).clone();
            hovered_col.set(Some(col));
            predicted_pos.set(b.predict_disc(col));
        })
    };
    let handle_mouseout = {
        let hovered_col = hovered_col.clone();
        let predicted_pos = predicted_pos.clone();
        Callback::from(move |_col: usize| {
            hovered_col.set(None);
            predicted_pos.set(None);
        })
    };

    let pixel_size = "80px";
    let grid_style = format!(
        "display: grid; text-align: center; grid-template-columns: repeat({}, {}); grid-auto-rows: {};",
        board.cols, pixel_size, pixel_size
    );

    let cell_style_hovered = "
    background-color: lightgray;
    box-shadow: 2px 2px 5px rgba(0, 0, 0, 0.3);
    ";

    html! {
        <>
            <Link<Route> to={Route::Home}>
                <button class="btn-back">
                    <span class="circle" aria-hidden="true">
                        <span class="icon arrow"></span>
                    </span>
                    <span class="button-text">{"Back to Home"}</span>
                </button>
            </Link<Route>>
            <h2 class="text_titles">{ "Connect Four" }</h2>

            <h2 class="radio-buttons" style="display: flex; align-items: center;">
                { format!("Status: ") }
                { format!("Player1 - ") }
                <img src={players_icon} width="75" height="75" />
                { format!(", Player2/Computer - ") }
                <img src={comp_icon} width="75" height="75" />
            </h2>
            <div class="radio-buttons" style="display: flex; align-items: center;">
                <text>{ "Robot: "}</text>
                <label class="l-radio" for="none">
                    <input type="radio" id="none" name="difficulty" value="none" onclick={on_difficulty_change.reform(move |_| "none")} checked={app_state_borrowed.difficulty == Difficulty::None} />
                    <span>{"none"}</span>
                </label>
                <label class="l-radio" for="easy">
                    <input type="radio" id="easy" name="difficulty" value="easy" onclick={on_difficulty_change.reform(move |_| "easy")} checked={app_state_borrowed.difficulty == Difficulty::Easy} />
                    <span>{"easy"}</span>
                </label>
                <label class="l-radio" for="hard">
                    <input type="radio" id="hard" name="difficulty" value="hard" onclick={on_difficulty_change.reform(move |_| "hard")} checked={app_state_borrowed.difficulty == Difficulty::Hard}/>
                    <span>{"hard"}</span>
                </label>
            </div>
            <p class="radio-buttons" style="display: flex; align-items: center;">
                { format!("Current turn: ") }
                <img src={current_player_icon} width="30" height="30" />
                { format!(" ({})", current_player) }
            </p>

            <div class="container-connect4">
                <div class="grid" style={grid_style.clone()}>
                    {
                        for board.grid.iter().enumerate().map(|(row, line)| {
                            html! {
                                {
                                    for line.iter().enumerate().map(|(col, &cell)| {
                                        let mut cell_style: &str = "";
                                        if let Some(hovered_col) = *hovered_col {
                                            if hovered_col == col {
                                                cell_style = cell_style_hovered;
                                            }
                                        };
                                        let is_enabled = matches!(board.state, connect4::State::Running);
                                        // robot move
                                        if *player1_done == true {
                                            on_column_click_comp.emit(col);
                                        }
                                        html! {
                                            <button
                                            class="cell"
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
                                                        connect4::Cell::Empty => {
                                                            // UI prediction of the future position of next move
                                                            if let Some(pos) = *predicted_pos {

                                                                if row == pos.0 && col == pos.1 && is_enabled { html! { <img src={current_player_icon} style="opacity:0.6; transform: translate(-13px, -8px);" width="80" height="80"/> } }

                                                                else {html! {}}
                                                            } else {
                                                                html! {}
                                                            }
                                                        },

                                                        connect4::Cell::Occupied(Player::Red) => html! { <img src={players_icon} style="transform: translate(-13px, -8px);" width="80" height="80" /> },
                                                        connect4::Cell::Occupied(Player::Yellow) => html! { <img src={comp_icon} style="transform: translate(-13px, -8px);" width="80" height="80" /> },

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

////////////////// toot otto
#[function_component(TootOttoGame)]
fn toot_otto_game() -> Html {
    let mut app_state_borrowed = APP_STATE.lock().unwrap();

    // Load state when the application starts
    let loaded_state = load_state();
    if let Some(state) = loaded_state {
        *app_state_borrowed = state;
    }

    let board = use_state(|| TootBoard::new(4, 6)); // Standard TOOT-OTTO board size

    // State to keep track of the currently selected piece
    let selected_piece = use_state(|| None);

    let on_piece_select = {
        let selected_piece = selected_piece.clone();
        Callback::from(move |piece: Piece| {
            selected_piece.set(Some(piece));
        })
    };

    // let app_state_borrowed = APP_STATE.lock().unwrap();
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

            // Save state when the difficulty changes
            save_state(&app_state);
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
            let current_difficulty = app_state_borrowed.difficulty; // Directly accessing because we cloned the state.

            match current_difficulty {
                Difficulty::None => (), // Do nothing if no difficulty is set.
                Difficulty::Easy => {
                    if let Err(e) = b.computer_move() {
                        println!("Error: {}", e);
                    } else {
                        board.set(b);
                        player1_done.set(false); // Reset the player1_done flag.
                    }
                }
                // Difficulty::Hard => {
                //     if let Err(e) = b.computer_move_hard(_col) {
                //         println!("Error: {}", e);
                //     } else {
                //         println!("Hard moved");
                //         board.set(b);
                //         player1_done.set(false); // Reset the player1_done flag.
                //     }
                // }
                Difficulty::Hard => {
                    if let Some((_row, col)) = b.last_move {
                        if let Err(e) = b.computer_move_hard(col) {
                            println!("Error: {}", e);
                        } else {
                            println!("Hard moved");
                            board.set(b);
                            player1_done.set(false);
                        }
                    } else {
                        if let Err(e) = b.computer_move_hard(_col) {
                            println!("Error: {}", e);
                        } else {
                            println!("Hard moved");
                            board.set(b);
                            player1_done.set(false);
                        }
                    }
                }

            }
        })
    };

    let predicted_pos: UseStateHandle<Option<(usize, usize)>> = use_state(|| None);

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
        let board = board.clone();
        let selected_piece = selected_piece.clone();
        let hovered_col = hovered_col.clone();
        let predicted_pos = predicted_pos.clone();
        Callback::from(move |col: usize| {
            let b = (*board).clone();
            if let Some(_piece) = *selected_piece {
                hovered_col.set(Some(col));
                predicted_pos.set(b.predict_piece(col));
            } else {
                hovered_col.set(None);
                predicted_pos.set(None);
            }
        })
    };
    let handle_mouseout = {
        let hovered_col = hovered_col.clone();
        let predicted_pos = predicted_pos.clone();
        Callback::from(move |_col: usize| {
            hovered_col.set(None);
            predicted_pos.set(None);
        })
    };

    let pixel_size = "80px"; // Smaller pieces for a more complex board
    let grid_style = format!(
        "display: grid; text-align: center; grid-template-columns: repeat({}, {}); grid-auto-rows: {};",
        board.cols, pixel_size, pixel_size
    );
    let current_player = match board.current_turn {
        TootPlayer::Toot => "TOOT",
        TootPlayer::Otto => "OTTO",
    };

    let cell_style_hovered = "
    background-color: lightgray;
    box-shadow: 2px 2px 5px rgba(0, 0, 0, 0.3);
    ";
    let btn_style_regular: &str = "color: dimgray; margin: 4px;";
    let btn_style_selected = "background-color: lightgray; color: black; margin: 4px;";

    let (player1_word, opponent_word) = match app_state_borrowed.player_as_toot_otto {
        PlayerAsTootOtto::PlayerToot => ("TOOT", "OTTO"),
        PlayerAsTootOtto::PlayerOtto => ("OTTO", "TOOT"),
    };
    html! {
        <>
            <Link<Route> to={Route::Home}>
                <button class="btn-back">
                    <span class="circle" aria-hidden="true">
                        <span class="icon arrow"></span>
                    </span>
                    <span class="button-text">{"Back to Home"}</span>
                </button>
            </Link<Route>>
            <h1 class="text_titles">{ "TOOT and OTTO" }</h1>
            <h2 class="radio-buttons" style="display: flex; align-items: center;">
                { format!("Status: ") }
                { format!("Player1 - ") }
                { player1_word }
                { format!(", Player2/Computer - ") }
                { opponent_word }
            </h2>
            <div class="radio-buttons" style="display: flex; align-items: center;">
                <text>{ "Robot: "}</text>
                <label class="l-radio" for="none">
                    <input type="radio" id="none" name="difficulty" value="none" onclick={on_difficulty_change.reform(move |_| "none")} checked={app_state_borrowed.difficulty == Difficulty::None} />
                    <span>{"none"}</span>
                </label>
                <label class="l-radio" for="easy">
                    <input type="radio" id="easy" name="difficulty" value="easy" onclick={on_difficulty_change.reform(move |_| "easy")} checked={app_state_borrowed.difficulty == Difficulty::Easy} />
                    <span>{"easy"}</span>
                </label>
                <label class="l-radio" for="hard">
                    <input type="radio" id="hard" name="difficulty" value="hard" onclick={on_difficulty_change.reform(move |_| "hard")} checked={app_state_borrowed.difficulty == Difficulty::Hard}/>
                    <span>{"hard"}</span>
                </label>
            </div>
            <h2>{ format!("Current turn: {}", current_player) }</h2>
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
            <div class="container-toototto">
                <div class="grid" style={grid_style.clone()}>
                    {
                        for board.grid.iter().enumerate().map(|(row, line)| {
                            html! {
                                {
                                    for line.iter().enumerate().map(|(col, &cell)| {
                                        let mut cell_style: &str = "text-align: center;";
                                        if let Some(hovered_col) = *hovered_col {
                                            if hovered_col == col {
                                                cell_style = cell_style_hovered;
                                            }
                                        };
                                        // robot move
                                        if *player1_done == true {
                                            on_column_click_comp.emit(col);
                                        }
                                        let is_enabled = matches!(board.state, toot_otto::State::Running);
                                        html! {
                                            <button
                                            class="cell"
                                            style={cell_style}
                                            onmouseenter={
                                                handle_mouseover.reform(move |_| col)
                                            }
                                            onmouseleave={handle_mouseout.reform(move |_| col)}
                                            onclick={on_column_click.reform(move |_| col)}
                                            disabled={!is_enabled}
                                            >
                                                {
                                                    match cell {
                                                        TootCell::Empty => {
                                                            // UI prediction of the future position of next move
                                                            if let Some(pos) = *predicted_pos {
                                                                if row == pos.0 && col == pos.1 {
                                                                    if let Some(piece) = *selected_piece {
                                                                        match piece {
                                                                            Piece::T => html! {<text style="font-size: 60px; text-align: center; display: block; line-height: 0.65; opacity: 0.6;">{"T"}</text>},
                                                                            Piece::O => html! {<text style="font-size: 60px; text-align: center; display: block; line-height: 0.65; opacity: 0.6;">{"O"}</text>},
                                                                        }
                                                                    } else {html! {<text>{" "}</text>}}
                                                                }
                                                                else {html! {<text>{" "}</text>}}
                                                            } else {
                                                                html! {<text>{" "}</text>}
                                                            }
                                                        },
                                                        TootCell::Occupied(piece) => match piece {
                                                            Piece::T => html! {<text style="font-size: 60px; text-align: center; display: block; line-height: 0.65; ">{"T"}</text>},
                                                            Piece::O => html! {<text style="font-size: 60px; text-align: center; display: block; line-height: 0.65; ">{"O"}</text>},
                                                        },
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
