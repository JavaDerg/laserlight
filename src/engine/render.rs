use web_sys::WebGlRenderingContext;

pub struct Renderer {
    ctx: WebGlRenderingContext,
}

impl Renderer {
    pub fn new(ctx: WebGlRenderingContext) -> Self {
        Self { ctx }
    }

    pub fn render(&mut self) {
        self.ctx.clear_color(0.0, 0.0, 0.0, 1.0);
        self.ctx.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);
    }
}
