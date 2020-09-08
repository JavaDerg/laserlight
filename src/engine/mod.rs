use crate::engine::render::Renderer;
use crate::err::{EngineError, ErrorConverter};
use builder::EngineBuilder;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, WebGlRenderingContext};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::Window;

pub mod builder;
mod render;
mod resource;

#[wasm_bindgen]
#[derive(Clone)]
pub struct Engine {}

impl Engine {
    pub(self) fn new() -> Self {
        Self {}
    }

    pub fn run(
        self,
        _builder: EngineBuilder,
        window: Window,
        canvas: HtmlCanvasElement,
        event_loop: EventLoop<()>,
    ) -> Result<(), EngineError> {
        use winit::event::{Event, WindowEvent};

        log::info!("Starting engine");

        let context = canvas
            .get_context("webgl")?
            .unwrap()
            .dyn_into::<WebGlRenderingContext>()
            .describe("Unable to obtain WebGL rendering context from canvas")?;

        let mut renderer = Renderer::new(context);

        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Poll;

            match event {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    window_id,
                } if window_id == window.id() => *control_flow = ControlFlow::Exit,
                Event::MainEventsCleared => {
                    // TODO: run game (js) update
                    if let Err(err) = renderer.update() {
                        // TODO: handle error
                    }
                    window.request_redraw();
                },
                Event::RedrawRequested(window_id) if window_id == window.id() => renderer.render(),
                _ => (),
            }
        });
    }
}
