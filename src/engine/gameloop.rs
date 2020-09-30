use imgui::{im_str, Condition, Window};

pub struct GameLoop {}

impl GameLoop {
    pub fn new() -> Self {
        GameLoop {}
    }

    pub fn update<C>(&mut self, _meta: &super::meta::EngineMeta, ui: &mut C)
    where
        C: FnMut(Box<dyn FnOnce(&imgui::Ui)>),
    {
        ui(Box::new(|ui| {
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
        }));
    }
}
