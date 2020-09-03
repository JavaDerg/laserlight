use wasm_bindgen::prelude::*;
//use crate::bindings::*;
use crate::engine::Engine;

#[wasm_bindgen]
pub struct EngineBuilder {
    game_name: String
}

impl EngineBuilder {
    pub fn new(game_name: String) -> Self {
        EngineBuilder {
            game_name
        }
    }

    pub fn build(self) -> Engine {
        Engine {
            game_name: self.game_name
        }
    }
}
