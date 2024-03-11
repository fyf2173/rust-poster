use image::{imageops, DynamicImage, Rgb, Rgba, RgbaImage};

use crate::core::core::meger_image_to_buffer;

use super::canvas_handler::IHandler;

pub struct RectangleHandler {
    pub x: u32,
    pub y: u32,
    pub with: u32,
    pub height: u32,
    pub color: Rgba<u8>,
}

impl IHandler for RectangleHandler {
    fn draw(&self, carrier: &mut image::ImageBuffer<Rgb<u8>, Vec<u8>>) {
        let mut painter = RgbaImage::new(self.with, self.height);
        imageops::vertical_gradient(&mut painter, &self.color, &self.color);

        meger_image_to_buffer(
            carrier,
            &DynamicImage::ImageRgba8(painter).into_rgb8(),
            self.x,
            self.y,
        );
    }
}
