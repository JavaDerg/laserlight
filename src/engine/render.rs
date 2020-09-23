use crate::engine::asrt;
use crate::engine::resource::PendingLoad;
use flume::Receiver;
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

        Ok(())
    }
}
