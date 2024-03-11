use image::{ImageBuffer, Rgb};
use imageproc::drawing::draw_line_segment_mut;

use super::canvas_handler::IHandler;

pub struct LineHandler {
    pub x1: f32,
    pub y1: f32,
    pub x2: f32,
    pub y2: f32,
    pub color: Rgb<u8>,
}

impl IHandler for LineHandler {
    fn draw(&self, carrier: &mut ImageBuffer<Rgb<u8>, Vec<u8>>) {
        draw_line_segment_mut(carrier, (self.x1, self.y1), (self.x2, self.y2), self.color);
    }
}
