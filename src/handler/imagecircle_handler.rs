use image::{imageops, DynamicImage, RgbaImage};

use crate::core::core::{self, image_to_circle, meger_image_to_buffer};

use super::canvas_handler::IHandler;

pub struct ImageCircleHandler {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub path: String,
}

impl IHandler for ImageCircleHandler {
    fn draw(&self, carrier: &mut RgbaImage) {
        let origin_img = image::open(&self.path).unwrap();
        let painter = core::resize_image(origin_img, self.width, self.height, imageops::Nearest);

        let circle_img = image_to_circle(painter.into());

        meger_image_to_buffer(
            carrier,
            &DynamicImage::ImageRgba8(circle_img),
            self.x,
            self.y,
        );
    }
}
