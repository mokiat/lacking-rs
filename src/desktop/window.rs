use super::*;
use crate::app;
use glfw::Context;
use std::error::Error;

pub struct Window {
    title: String,
}

impl Window {
    pub fn open(cfg: &Config) -> Result<(), Box<dyn Error>> {
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS)?;

        let (mut glfw_window, glfw_events) = glfw
            .create_window(
                cfg.width(),
                cfg.height(),
                cfg.title(),
                glfw::WindowMode::Windowed,
            )
            .ok_or_else(|| String::from("failed to create window"))?;

        glfw_window.set_key_polling(true);
        glfw_window.make_current();

        while !glfw_window.should_close() {
            glfw.poll_events();
            for (_, _) in glfw::flush_messages(&glfw_events) {
                // TODO: Handle events...
            }
        }

        Ok(())
    }
}

impl app::Window for Window {
    fn title(&self) -> &str {
        &self.title
    }

    fn close(&self) {}
}
