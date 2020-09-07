use crate::engine::Engine;
use crate::err::{EngineError, ErrorConverter};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct EngineBuilder {
    game_name: String,
}

impl EngineBuilder {
    pub fn new(game_name: String) -> Self {
        EngineBuilder { game_name }
    }

    pub fn run(self) -> Result<Engine, EngineError> {
        use winit::event_loop::EventLoop;
        use winit::platform::web::WindowExtWebSys;
        use winit::window::WindowBuilder;

        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_title(&self.game_name)
            .build(&event_loop)
            .describe("Unable to build Window")?;
        let canvas = window.canvas();
        canvas.style().set_css_text(
            r"
            position: fixed;
            top: 0;
            left: 0;
            width: 100%;
            height: 100%;
        ",
        );
        canvas.set_oncontextmenu(Some(&js_sys::Function::new_with_args(
            "ev",
            r"
            ev.preventDefault();
            return false;
        ",
        )));

        {
            let window = web_sys::window().describe("Windows is None?")?;
            let document = window.document().describe("Document is None?")?;
            let body = document.body().describe("Body is None?")?;
            body.append_child(&canvas)
                .describe("Was unable to append canvas to body")?;
        }

        log::info!("Created canvas");

        let engine = Engine::new();

        Engine::run(engine.clone(), self, window, canvas, event_loop)?;

        Ok(engine)
    }
}
