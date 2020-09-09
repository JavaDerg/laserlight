use crate::engine::resource::PendingLoad;
use crossbeam_queue::SegQueue;
use web_sys::WebGlRenderingContext;

pub struct Renderer {
    inner: Inner,
}

struct Inner {
    pub ctx: WebGlRenderingContext,
    pub resource_queue: SegQueue<PendingLoad>,
}

impl Renderer {
    pub fn new(ctx: WebGlRenderingContext) -> Self {
        Self {
            inner: Inner {
                ctx,
                resource_queue: SegQueue::new(),
            },
        }
    }

    pub fn update(&self) -> Result<(), crate::err::EngineError> {
        while let Ok(_item) = self.inner.resource_queue.pop() {}
        Ok(())
    }

    pub fn render(&self) -> Result<(), crate::err::EngineError> {
        let ctx = &self.inner.ctx;

        ctx.clear_color(0.0, 0.0, 0.0, 1.0);
        ctx.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);

        Ok(())
    }
}
