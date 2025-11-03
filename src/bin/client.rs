use leptos::mount_to_body;
use rustboard::app::App;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn hydrate() {
    mount_to_body(App);
}

pub fn main() {}
