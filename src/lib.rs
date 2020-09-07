use wasm_bindgen::prelude::*;

pub mod bindings;
pub mod engine;
pub mod err;

#[wasm_bindgen(start)]
pub fn main() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    console_log::init_with_level(log::Level::Debug).expect("Unable to initialize console logging");
}
