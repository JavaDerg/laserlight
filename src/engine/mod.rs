use crate::err::{EngineError, ErrorConverter};
use builder::EngineBuilder;
use imgui_winit_support::{HiDpiMode, WinitPlatform};
use std::time::Duration;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, WebGl2RenderingContext};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::Window;

mod asrt;
pub mod builder;
mod gameloop;
mod meta;
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
            .dyn_into::<WebGl2RenderingContext>()
            .describe("Unable to obtain WebGL rendering context from canvas")?;

        let mut imgui = imgui::Context::create();
        let mut platform = WinitPlatform::init(&mut imgui);
        platform.attach_window(imgui.io_mut(), &window, HiDpiMode::Default);

        let mut rt = asrt::Runtime::new();
        let handle = rt.get_handle();
        let mut meta = meta::EngineMeta::new();
        let mut renderer = render::Renderer::new(context);
        let mut gameloop = gameloop::GameLoop::new();

        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Poll;
            match event {
                Event::NewEvents(_) => {
                    meta.update_delta();
                    imgui.io_mut().update_delta_time(*meta.delta_dur())
                }
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    window_id,
                } if window_id == window.id() => *control_flow = ControlFlow::Exit,
                Event::MainEventsCleared => {
                    meta.update();

                    window.request_redraw();
                }
                Event::RedrawRequested(window_id) if window_id == window.id() => {
                    if let Err(err) = platform.prepare_frame(imgui.io_mut(), &window) {
                        log::error!("{}", err);
                        // TODO: check if error is recoverable
                        *control_flow = ControlFlow::Exit;
                    }

                    let ui = imgui.frame();
                    gameloop.update(&meta, &ui);

                    rt.step_min_time(Duration::from_millis(1));

                    if let Err(err) = renderer.update(&meta, &handle) {
                        log::error!("{}", err);
                        // TODO: check if error is recoverable
                        *control_flow = ControlFlow::Exit;
                    }

                    platform.prepare_render(&ui, &window);
                    let draw_data = ui.render();
                    if let Err(err) = renderer.render(draw_data) {
                        log::error!("{}", err);
                        // TODO: check if error is recoverable
                        *control_flow = ControlFlow::Exit;
                    }
                }
                event => {
                    platform.handle_event(imgui.io_mut(), &window, &event);
                    // TODO: Handle input events
                }
            }
        });
    }
}
