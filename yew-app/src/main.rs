use yew::prelude::*;
// use yew::events::InputData;
use yew_router::prelude::*;
mod connect4;
use connect4::{Board, Player, State, Cell};

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

#[derive(Debug, PartialEq, Clone, Copy)]
enum CompIcon {
    Option3,
    Option4,
}

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

// type SharedAppState = Rc<RefCell<AppState>>;
// #[derive(Properties, Clone, PartialEq)]
// pub struct GameProps {
//     pub app_state: SharedAppState,
// }

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
            <h1>{ "Welcome to Connect Four!" }</h1>
            <p>{ "This is a simple implementation of the Connect Four game using Yew." }</p>
            <div>
                <p>{ "Select the difficulty of computer player:" }</p>     
                <input type="radio" id="easy" name="difficulty" value="easy" onclick={on_difficulty_change.clone()} checked={app_state.difficulty == Difficulty::Easy} />
                <label for="easy">{"Easy"}</label>
                <input type="radio" id="hard" name="difficulty" value="hard" onclick={on_difficulty_change.clone()} checked={app_state.difficulty == Difficulty::Hard}/>
                <label for="hard">{"Hard"}</label>
            </div>
            <div>
                <p>{ "Select the player icon you want:" }</p>     
                <input type="radio" id="option1" name="player_icon" value="option1" onclick={on_player_icon_change.clone()} checked={app_state.player_icon == PlayerIcon::Option1} />
                <label for="option1">{"option1"}</label>
                <input type="radio" id="option2" name="player_icon" value="option2" onclick={on_player_icon_change} checked={app_state.player_icon == PlayerIcon::Option2} />
                <label for="option2">{"option2"}</label>
            </div>
            <div>
                <p>{ "Select the computer icon you want:" }</p>     
                <input type="radio" id="option3" name="comp_icon" value="option3" onclick={on_comp_icon_change.clone()} checked={app_state.comp_icon == CompIcon::Option3} />
                <label for="option3">{"option3"}</label>
                <input type="radio" id="option4" name="comp_icon" value="option4" onclick={on_comp_icon_change} checked={app_state.comp_icon == CompIcon::Option4} />
                <label for="option4">{"option4"}</label>
            </div>
            <p>{ "Click the button below to start playing!" }</p>
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
                                    connect4::Cell::Occupied(Player::Red) => html! { <img src="https://raw.githubusercontent.com/kooner27/421_projects/main/yew-app/static/armor.png" /> },
                                    connect4::Cell::Occupied(Player::Yellow) => html! { <img src="https://raw.githubusercontent.com/kooner27/421_projects/main/yew-app/static/sword.png" /> },
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
                        State::Won(player) => html! { <p>{ format!("Player {:?} wins!", player) }</p> },
                        State::Draw => html! { <p>{ "The game is a draw!" }</p> },
                        State::Running => html! { <p>{ "Game is in progress..." }</p> },
                    }
                }
            </div>
        </>
    }
}


////////////////// toot otto










