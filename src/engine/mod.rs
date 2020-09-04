use wasm_bindgen::prelude::*;
use crate::err::{EngineError, ErrorConverter};
use builder::EngineBuilder;
use wasm_bindgen::__rt::std::time::Instant;
use web_sys::HtmlCanvasElement;
use winit::event_loop::{EventLoop, ControlFlow};
use winit::window::Window;

pub mod builder;
mod render;

#[wasm_bindgen]
#[derive(Clone)]
pub struct Engine {
}

impl Engine {
    pub(in self) fn new() -> Self {
        Self {

        }
    }

    pub fn run(
        self,
        builder: EngineBuilder,
        window: Window,
        canvas: HtmlCanvasElement,
        event_loop: EventLoop<()>
    ) -> Result<(), EngineError> {
        use winit::event::{Event, WindowEvent};

        console_log::init_with_level(log::Level::Debug);
        log::info!("Starting engine");

        log::info!("Created canvas");

        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Wait;

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

        Ok(())
    }
}
