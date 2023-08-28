use windows::{core::Result, Foundation::Numerics::Vector2};

pub struct App {}

impl App {
    pub fn new() -> Self {
        Self {}
    }
    pub fn on_pointer_moved(&mut self, _point: &Vector2) -> Result<()> {
        Ok(())
    }

    pub fn on_parent_size_changed(&mut self, _new_size: &Vector2) -> Result<()> {
        Ok(())
    }

    pub fn on_pointer_pressed(&mut self, _is_right_button: bool, _is_eraser: bool) -> Result<()> {
        Ok(())
    }
}
