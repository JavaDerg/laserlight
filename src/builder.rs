use wasm_bindgen::prelude::*;
use crate::bindings::*;

#[wasm_bindgen]
pub struct EngineBuilder {

}

impl EngineBuilder {
    pub fn new() -> EngineBuilder {
        lockNavigation(true);
        EngineBuilder {
        }
    }
}
