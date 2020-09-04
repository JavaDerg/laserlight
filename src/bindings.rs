use wasm_bindgen::prelude::*;
use crate::engine::builder::EngineBuilder;
use crate::engine::Engine;
use crate::err::EngineError;

#[wasm_bindgen]
pub fn new_engine_builder(game_name: String) -> EngineBuilder {
    EngineBuilder::new(game_name)
}

#[wasm_bindgen]
pub fn start_engine(builder: EngineBuilder) ->  Result<Engine, JsValue> {
    builder.run().map_err(|err| err.into())
}
