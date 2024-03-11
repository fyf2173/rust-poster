use image::{imageops, DynamicImage, ImageBuffer, Rgb};
use imageproc::drawing::{draw_filled_circle_mut, draw_filled_ellipse_mut, draw_hollow_circle_mut};

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
    fn draw(&self, carrier: &mut ImageBuffer<Rgb<u8>, Vec<u8>>) {
        let origin_img = image::open(&self.path).unwrap();
        let painter = core::resize_image(origin_img, self.width, self.height, imageops::Nearest);

        let circle_img = image_to_circle(painter);

        meger_image_to_buffer(
            carrier,
            &DynamicImage::ImageRgba8(circle_img).into_rgb8(),
            self.x,
            self.y,
        );
    }
}
