use wasm_bindgen::prelude::*;
use crate::err::{EngineError, ErrorConverter};
use builder::EngineBuilder;
use wasm_bindgen::__rt::std::time::Instant;

pub mod builder;

#[wasm_bindgen]
pub struct Engine {
    pub(self) game_name: String
}

impl Engine {
    pub fn run(&mut self) -> Result<(), EngineError> {
        use winit::window::WindowBuilder;
        use winit::platform::web::WindowExtWebSys;
        use winit::event_loop::{EventLoop, ControlFlow};
        use winit::event::{Event, WindowEvent};

        console_log::init_with_level(log::Level::Debug);
        log::info!("Starting engine");

        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_title(&self.game_name)
            .build(&event_loop)
            .describe("Unable to build Window")?;
        let canvas = window.canvas();
        canvas.style().set_css_text(r"
            position: fixed;
            top: 0;
            left: 0;
            width: 100%;
            height: 100%;
        ");
        canvas.set_oncontextmenu(Some(&js_sys::Function::new_with_args("ev", r"
            ev.preventDefault();
            return false;
        ")));

        {
            let window = web_sys::window().describe("Windows is None?")?;
            let document = window.document().describe("Document is None?")?;
            let body = document.body().describe("Body is None?")?;
            body.append_child(&canvas).describe("Was unable to append canvas to body")?;
        }

        log::info!("Created canvas");

        event_loop.run(move |event, _, control_flow| {
            // *control_flow = ControlFlow::Wait;

            match event {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    window_id,
                }
                if window_id == window.id() => *control_flow = ControlFlow::Exit,
                Event::MainEventsCleared => {
                    log::info!("Draw! {:?}", Instant::now());
                    window.request_redraw();
                }
                _ => (),
            }
        });
    }
}
