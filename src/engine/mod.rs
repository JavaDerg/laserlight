use crate::engine::render::Renderer;
use crate::err::{EngineError, ErrorConverter};
use builder::EngineBuilder;
use std::future::Future;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::__rt::core::pin::Pin;
use wasm_bindgen::__rt::core::task::{Context, Poll};
use wasm_bindgen::__rt::core::time::Duration;
use web_sys::{HtmlCanvasElement, WebGlRenderingContext};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::Window;

mod asrt;
pub mod builder;
mod render;
mod resource;

#[wasm_bindgen]
#[derive(Clone)]
pub struct Engine {}

struct ForeverFuture(fn() -> ());
impl Future for ForeverFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.0();
        cx.waker().wake_by_ref();
        Poll::Pending
    }
}

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

        let mut rt = asrt::Runtime::new();
        let handle = rt.get_handle();
        let renderer = Renderer::new(context);

        rt.spawn(ForeverFuture(|| log::warn!("Nice!")));

        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Poll;

            match event {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    window_id,
                } if window_id == window.id() => *control_flow = ControlFlow::Exit,
                Event::MainEventsCleared => {
                    // TODO: run game (js) update
                    rt.step_min_time(Duration::from_millis(1));
                    if let Err(err) = renderer.update(&handle) {
                        log::error!("{}", err);
                        // TODO: check if error is recoverable
                        *control_flow = ControlFlow::Exit;
                    }
                    window.request_redraw();
                }
                Event::RedrawRequested(window_id) if window_id == window.id() => {
                    if let Err(err) = renderer.render() {
                        log::error!("{}", err);
                        // TODO: check if error is recoverable
                        *control_flow = ControlFlow::Exit;
                    }
                }
                _ => (),
            }
        });
    }
}
