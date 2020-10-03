pub mod shader;
mod imgui_render;

use crate::engine::asrt;
use crate::engine::resource::PendingLoad;
use flume::Receiver;
use glow::HasContext;
use web_sys::WebGl2RenderingContext;

#[macro_export]
macro_rules! glc {
    ($ctx:expr, $any:expr) => {
        unsafe {
            #[cfg(debug_assertions)]
            while $ctx.get_error() != glow::NO_ERROR {}
            let out = $any;
            #[cfg(debug_assertions)]
            while match $ctx.get_error() {
                glow::NO_ERROR => false,
                err => {
                    log::error!("[OpenGL Error] {}", err);
                    true
                }
            } {}
            out
        }
    };
}

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

        glc!(ctx, ctx.clear_color(0.1, 0.1, 0.1, 1.0));
        glc!(ctx, ctx.clear(glow::COLOR_BUFFER_BIT));

        // https://github.com/ocornut/imgui/blob/7b1ab5b27586a3b297aac336d6a97873b11d4078/examples/imgui_impl_opengl3.cpp#L294

        Ok(())
    }
}
