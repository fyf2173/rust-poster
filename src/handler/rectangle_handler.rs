use image::{imageops, Rgba, RgbaImage};

use super::canvas_handler::IHandler;

pub struct RectangleHandler {
    pub x: u32,
    pub y: u32,
    pub with: u32,
    pub height: u32,
    pub color: Rgba<u8>,
}

impl IHandler for RectangleHandler {
    fn draw(&self, carrier: &mut RgbaImage) {
        let mut painter = RgbaImage::new(self.with, self.height);
        imageops::vertical_gradient(&mut painter, &self.color, &self.color);

        imageops::overlay(carrier, &painter, self.x as i64, self.y as i64);
    }
}
