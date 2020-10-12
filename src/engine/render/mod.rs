pub mod gl_consts;
mod imgui_render;
pub mod shader;

use crate::engine::asrt;
use crate::engine::resource::PendingLoad;
use flume::Receiver;
use web_sys::WebGlRenderingContext;

#[macro_export]
macro_rules! glc {
    ($ctx:expr, $any:expr) => {
        unsafe {
            #[cfg(debug_assertions)]
            while $ctx.get_error() != $crate::engine::render::gl_consts::NO_ERROR {}
            let out = $any;
            #[cfg(debug_assertions)]
            while match $ctx.get_error() {
                $crate::engine::render::gl_consts::NO_ERROR => false,
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
    pub ctx: WebGlRenderingContext,
    pub resource_queue: (flume::Sender<PendingLoad>, Receiver<PendingLoad>),
    imgui_render: Option<imgui_render::ImguiRender>,
}

impl Renderer {
    pub fn new(ctx: WebGlRenderingContext) -> Self {
        Self {
            inner: Inner {
                ctx,
                resource_queue: flume::channel(),
                imgui_render: None,
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

    #[inline]
    pub fn pre_render(&mut self, imctx: &mut imgui::Context) {
        if self.inner.imgui_render.is_none() {
            self.inner.imgui_render = Some(imgui_render::ImguiRender::new(&self.inner.ctx, imctx));
        }
    }

    pub fn render(&mut self, mut ui: imgui::Ui) -> Result<(), crate::err::EngineError> {
        let ctx = &self.inner.ctx;

        glc!(ctx, ctx.clear_color(0.1, 0.1, 0.1, 1.0));
        glc!(ctx, ctx.clear(gl_consts::COLOR_BUFFER_BIT));

        self.inner
            .imgui_render
            .as_ref()
            .expect("you need to call pre_render")
            .draw(ctx, ui);

        Ok(())
    }
}
