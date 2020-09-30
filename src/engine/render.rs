use crate::engine::asrt;
use crate::engine::resource::PendingLoad;
use flume::Receiver;
use glow::HasContext;
use web_sys::WebGl2RenderingContext;

pub struct Renderer {
    inner: Inner,
}

struct Inner {
    pub ctx: glow::Context,
    pub resource_queue: (flume::Sender<PendingLoad>, Receiver<PendingLoad>),
}

impl Renderer {
    pub fn new(ctx: WebGl2RenderingContext) -> Self {
        Self {
            inner: Inner {
                ctx: glow::Context::from_webgl2_context(ctx),
                resource_queue: flume::channel(),
            },
        }
    }

    pub fn update(
        &mut self,
        _meta: &super::meta::EngineMeta,
        _handle: &asrt::Handle,
    ) -> Result<(), crate::err::EngineError> {
        self.inner.resource_queue.1.try_iter().for_each(|_| ());
        Ok(())
    }

    pub fn render(&mut self, imgui_draw: &imgui::DrawData) -> Result<(), crate::err::EngineError> {
        let ctx = &self.inner.ctx;

        // TODO: imgui_draw

        unsafe {
            ctx.clear_color(0.1, 0.1, 0.1, 1.0);
            ctx.clear(glow::COLOR_BUFFER_BIT);
        }

        Ok(())
    }
}
