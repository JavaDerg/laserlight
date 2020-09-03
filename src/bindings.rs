use wasm_bindgen::prelude::*;
use crate::engine::builder::EngineBuilder;
use crate::engine::Engine;
use crate::err::EngineError;

#[wasm_bindgen]
pub fn new_engine_builder(game_name: String) -> EngineBuilder {
    EngineBuilder::new(game_name)
}

#[wasm_bindgen]
pub fn build_engine(builder: EngineBuilder) -> Engine {
    builder.build()
}

#[wasm_bindgen]
pub fn run_engine(engine: &mut Engine) -> Result<(), JsValue> {
    engine.run().map_err(|err| err.into())
}
