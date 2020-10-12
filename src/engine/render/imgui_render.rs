/*
    _/\_
  _/    \
_/  o    |__________________
|--                        \===========
\_______                   x/
        \  __   ____   __  /
        | |  | |    | |  | |
        | |  | |    | |  | |
        | |  | |    | |  | |
        |_|  |_|    |_|  |_|

This is cat the cat
 */

use super::shader; // Module
use crate::glc;
use crate::shader; // Macro
use web_sys::WebGlRenderingContext;

use super::gl_consts as glcl;

pub struct ImguiRender {
    shader: shader::ShaderProgram,
    uniform_tex: web_sys::WebGlUniformLocation,
    uniform_proj_mtx: web_sys::WebGlUniformLocation,
    attrib_vtx_pos: u32,
    attrib_vtx_uv: u32,
    attrib_vtx_color: u32,
    tex: u32,
    vbo: u32,
    elements: u32,
}

impl ImguiRender {
    pub fn new(ctx: &WebGlRenderingContext, imctx: &mut imgui::Context) -> Self {
        let shader = shader!("imgui")
            .build(ctx)
            .expect("Imgui shader failed to build");

        let uniform_tex = glc!(ctx, ctx.get_uniform_location(shader.program, "Texture"))
            .expect("Failed to get uniform 'Texture' of imgui shader");
        let uniform_proj_mtx = glc!(ctx, ctx.get_uniform_location(shader.program, "ProjMtx"))
            .expect("Failed to get uniform 'ProjMtx' of imgui shader");
        let attrib_vtx_pos = glc!(ctx, ctx.get_attrib_location(shader.program, "Position"))
            .expect("Failed to get attrib location 'Position' of imgui shader");
        let attrib_vtx_uv = glc!(ctx, ctx.get_attrib_location(shader.program, "UV"))
            .expect("Failed to get attrib location 'UV' of imgui shader");
        let attrib_vtx_color = glc!(ctx, ctx.get_attrib_location(shader.program, "Color"))
            .expect("Failed to get attrib location 'Color' of imgui shader");
        let vbo =
            glc!(ctx, ctx.create_buffer()).expect("Failed to greate elements buffer for imgui");
        let elements =
            glc!(ctx, ctx.create_buffer()).expect("Failed to greate vbo buffer for imgui");

        let tex = glc!(ctx, ctx.create_texture()).expect("Failed to create font texture for imgui");
        glc!(ctx, ctx.bind_texture(glcl::TEXTURE_2D, Some(tex)));
        glc!(
            ctx,
            ctx.tex_parameter_i32(
                glcl::TEXTURE_2D,
                glcl::TEXTURE_MIN_FILTER,
                glcl::LINEAR as i32
            )
        );
        glc!(
            ctx,
            ctx.tex_parameter_i32(
                glcl::TEXTURE_2D,
                glcl::TEXTURE_MAG_FILTER,
                glcl::LINEAR as i32
            )
        );
        let mut fonts = imctx.fonts();
        let font_tex = fonts.build_rgba32_texture();
        glc!(
            ctx,
            ctx.tex_image_2d(
                glcl::TEXTURE_2D,
                0,
                glcl::RGBA as i32,
                font_tex.width as i32,
                font_tex.height as i32,
                0 as i32,
                glcl::RGBA,
                glcl::UNSIGNED_BYTE,
                Some(font_tex.data)
            )
        );

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
    pub fn draw(&self, ctx: &WebGlRenderingContext, ui: imgui::Ui) {
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

    pub fn drop(self, ctx: &WebGlRenderingContext) {
        glc!(ctx, ctx.delete_buffer(self.vbo));
        glc!(ctx, ctx.delete_buffer(self.elements));
        glc!(ctx, ctx.delete_texture(self.tex));
        self.shader.drop(ctx);
    }
}

pub fn capture(ctx: &WebGlRenderingContext) -> impl FnOnce(&WebGlRenderingContext) -> () {
    // Capture gl state
    let active_texture = glc!(ctx, ctx.get_parameter_i32(glcl::ACTIVE_TEXTURE));
    glc!(ctx, ctx.active_texture(glcl::TEXTURE0));
    let program = glc!(ctx, ctx.get_parameter_i32(glcl::CURRENT_PROGRAM));
    let texture = glc!(ctx, ctx.get_parameter_i32(glcl::TEXTURE_BINDING_2D));
    let array_buffer = glc!(ctx, ctx.get_parameter_i32(glcl::ARRAY_BUFFER_BINDING));
    let vertex_array_obj = glc!(ctx, ctx.get_parameter_i32(glcl::VERTEX_ARRAY_BINDING));
    let viewport = [
        glc!(ctx, ctx.get_parameter_indexed_i32(glcl::VIEWPORT, 0)),
        glc!(ctx, ctx.get_parameter_indexed_i32(glcl::VIEWPORT, 1)),
        glc!(ctx, ctx.get_parameter_indexed_i32(glcl::VIEWPORT, 2)),
        glc!(ctx, ctx.get_parameter_indexed_i32(glcl::VIEWPORT, 3)),
    ];
    let scissor_box = [
        glc!(ctx, ctx.get_parameter_indexed_i32(glcl::SCISSOR_BOX, 0)),
        glc!(ctx, ctx.get_parameter_indexed_i32(glcl::SCISSOR_BOX, 1)),
        glc!(ctx, ctx.get_parameter_indexed_i32(glcl::SCISSOR_BOX, 2)),
        glc!(ctx, ctx.get_parameter_indexed_i32(glcl::SCISSOR_BOX, 3)),
    ];
    let blend_src_rgb = glc!(ctx, ctx.get_parameter_i32(glcl::BLEND_SRC_RGB));
    let blend_dst_rgb = glc!(ctx, ctx.get_parameter_i32(glcl::BLEND_DST_RGB));
    let blend_src_alpha = glc!(ctx, ctx.get_parameter_i32(glcl::BLEND_SRC_ALPHA));
    let blend_dst_alpha = glc!(ctx, ctx.get_parameter_i32(glcl::BLEND_DST_ALPHA));
    let blend_equation_rgb = glc!(ctx, ctx.get_parameter_i32(glcl::BLEND_EQUATION_RGB));
    let blend_equation_alpha = glc!(ctx, ctx.get_parameter_i32(glcl::BLEND_EQUATION_ALPHA));
    let blend = glc!(ctx, ctx.is_enabled(glcl::BLEND));
    let cull_face = glc!(ctx, ctx.is_enabled(glcl::CULL_FACE));
    let depth_test = glc!(ctx, ctx.is_enabled(glcl::DEPTH_TEST));
    let scissor_test = glc!(ctx, ctx.is_enabled(glcl::SCISSOR_TEST));

    // Restore gl state
    move |ctx| {
        glc!(ctx, ctx.use_program(Some(program)));
    }
}
