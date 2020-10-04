use crate::shader; // Macro
use super::shader; // Module
use crate::glc;

pub struct ImguiRender {
    shader: shader::ShaderProgram,
    uniform_tex: web_sys::WebGlUniformLocation,
    uniform_proj_mtx: web_sys::WebGlUniformLocation,
    attrib_vtx_pos: u32,
    attrib_vtx_uv: u32,
    attrib_vtx_color: u32,
    tex: glow::WebTextureKey,
    vbo: glow::WebBufferKey,
    elements: glow::WebBufferKey,
}

impl ImguiRender {
    pub fn new(ctx: &glow::Context, imctx: &mut imgui::Context) -> Self {
        let shader = shader!("imgui").build(ctx).expect("Imgui shader failed to build");

        let uniform_tex = glc!(ctx, ctx.get_uniform_location(shader.program, "Texture")).expect("Failed to get uniform 'Texture' of imgui shader");
        let uniform_proj_mtx = glc!(ctx, ctx.get_uniform_location(shader.program, "ProjMtx")).expect("Failed to get uniform 'ProjMtx' of imgui shader");
        let attrib_vtx_pos = glc!(ctx, ctx.get_attrib_location(shader.program, "Position")).expect("Failed to get attrib location 'Position' of imgui shader");
        let attrib_vtx_uv = glc!(ctx, ctx.get_attrib_location(shader.program, "UV")).expect("Failed to get attrib location 'UV' of imgui shader");
        let attrib_vtx_color = glc!(ctx, ctx.get_attrib_location(shader.program, "Color")).expect("Failed to get attrib location 'Color' of imgui shader");
        let vbo = glc!(ctx, ctx.create_buffer()).expect("Failed to greate elements buffer for imgui");
        let elements = glc!(ctx, ctx.create_buffer()).expect("Failed to greate vbo buffer for imgui");

        let tex = glc!(ctx, ctx.create_texture()).expect("Failed to create font texture for imgui");
        glc!(ctx, ctx.bind_texture(glow::TEXTURE_2D, Some(tex)));
        glc!(ctx, ctx.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MIN_FILTER, glow::LINEAR as i32));
        glc!(ctx, ctx.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MAG_FILTER, glow::LINEAR as i32));
        let mut fonts = imctx.fonts();
        let font_tex = fonts.build_rgba32_texture();
        glc!(ctx, ctx.tex_image_2d(glow::TEXTURE_2D, 0, glow::RGBA as i32, font_tex.width as i32, font_tex.height as i32, 0 as i32, glow::RGBA, glow::UNSIGNED_BYTE, Some(font_tex.data)));

        Self {
            uniform_tex,
            uniform_proj_mtx,
            attrib_vtx_pos,
            attrib_vtx_uv,
            attrib_vtx_color,
            vbo,
            elements,
            tex,
            shader,
        }
    }

    // https://github.com/ocornut/imgui/blob/7b1ab5b27586a3b297aac336d6a97873b11d4078/examples/imgui_impl_opengl3.cpp#L294
    pub fn draw(&self, ctx: &glow::Context, ui: imgui::Ui) {
        let dd = ui.render();
        let fbw = (dd.display_size[0] * dd.framebuffer_scale[0]) as i32;
        let fbh = (dd.display_size[1] * dd.framebuffer_scale[1]) as i32;
        if fbw <= 0 || fbh <= 0 {
            return;
        }

        let restore = capture(ctx);

        todo!();

        restore(ctx);
    }


    pub fn drop(self, ctx: &glow::Context) {
        glc!(ctx, ctx.delete_buffer(self.vbo));
        glc!(ctx, ctx.delete_buffer(self.elements));
        glc!(ctx, ctx.delete_texture(self.tex));
        self.shader.drop(ctx);
    }
}

pub fn capture(ctx: &glow::Context) -> fn(&glow::Context) -> () {
    // Capture gl state
    todo!();

    // Restore gl state
    move |ctx| {
        todo!()
    }
}
