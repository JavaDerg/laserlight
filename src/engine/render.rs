use crate::engine::asrt;
use crate::engine::resource::PendingLoad;
use flume::Receiver;
use imgui::{Condition, Window};
use web_sys::WebGlRenderingContext;

pub struct Renderer {
    inner: Inner,
}

struct Inner {
    pub ctx: WebGlRenderingContext,
    pub resource_queue: (flume::Sender<PendingLoad>, Receiver<PendingLoad>),
}

impl Renderer {
    pub fn new(ctx: WebGlRenderingContext) -> Self {
        Self {
            inner: Inner {
                ctx,
                resource_queue: flume::channel(),
            },
        }
    }

    pub fn update(&self, handle: &asrt::Handle) -> Result<(), crate::err::EngineError> {
        self.inner.resource_queue.1.try_iter().for_each(|_| ());
        Ok(())
    }

    pub fn render(&self) -> Result<(), crate::err::EngineError> {
        let ctx = &self.inner.ctx;

        ctx.clear_color(0.0, 0.0, 0.0, 1.0);
        ctx.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);

        Window::new(im_str!("Hello world"))
            .size([300.0, 100.0], Condition::FirstUseEver)
            .build(&ui, || {
                ui.text(im_str!("Hello world!"));
                ui.text(im_str!("こんにちは世界！"));
                ui.text(im_str!("This...is...imgui-rs!"));
                ui.separator();
                let mouse_pos = ui.io().mouse_pos;
                ui.text(format!(
                    "Mouse Position: ({:.1},{:.1})",
                    mouse_pos[0], mouse_pos[1]
                ));
            });

        Ok(())
    }
}
