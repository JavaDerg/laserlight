use wasm_bindgen::prelude::*;
use crate::builder::EngineBuilder;

#[wasm_bindgen]
pub fn new_engine_builder() -> EngineBuilder {
    EngineBuilder::new()
}

#[wasm_bindgen]
extern {
    pub fn lockNavigation(lock: bool);
}
