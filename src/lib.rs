use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn greet_user(name: &str) {
    println!("Hello, {}!", name);
}
