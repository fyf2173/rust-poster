use image::{Rgba, RgbaImage};

use crate::core::{load_font, text_to_image};

use super::canvas_handler::IHandler;

pub struct TextHandler {
    pub title: String,
    pub x: i32,
    pub y: i32,
    pub color: Rgba<u8>,
    pub fontsize: f32,
    pub font: &'static [u8],
}

impl IHandler for TextHandler {
    fn draw(&self, carrier: &mut RgbaImage) {
        text_to_image(
            carrier,
            &self.title,
            self.x,
            self.y,
            self.color,
            self.fontsize,
            load_font(self.font),
        );
    }
}
