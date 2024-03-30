/*
The src/lib.rs file is the root of the Rust crate that we are compiling to WebAssembly. It
uses wasm-bindgen to interface with JavaScript. It imports the window.alert JavaScript function, and
exports the greet Rust function, which alerts a greeting message.

*/
// #[wasm_bindgen]
// pub fn greet() {
//     alert("Hello, connect4!");
// }


//* try these:
// get connect 4 layout: https://www.codewithfaraz.com/content/297/create-connect-four-game-using-html-css-and-javascript-source-code
// old connect 4: https://github.com/thinking-fun/Connect4-with-TootandOtto 
// html, css, js tidbits: https://www.shecodes.io/athena/62062-how-to-change-font-size-and-color-in-html


mod utils;

use wasm_bindgen::prelude::*;

extern crate is_prime;
use is_prime::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, wasm-is-prime!");
}


#[wasm_bindgen]
pub fn check_prime(s: &JsValue) -> u32 {
    let input: String = s.as_string().unwrap();
    match input.parse::<u32>() {
        Ok(num) => {
            if is_prime(&input) {
                alert("Input is Prime");
                return 1;
            } else {
                alert("Input is NOT Prime");
                return 0;
            }
        }
        Err(_) => {
            alert(&format!("Couldn't parse {}", input));
            return 0;
        }
    }
}
// fn is_prime(n: u32) -> u32 {
//     // add your code to check prime here
//     n % 2 // This line currently checks if a number is odd or even.
// }
