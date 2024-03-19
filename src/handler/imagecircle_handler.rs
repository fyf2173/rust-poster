use image::{imageops, RgbaImage};

use crate::core::{self, get_remote_resource, image_to_circle};

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

        imageops::overlay(carrier, &circle_img, self.x as i64, self.y as i64);
    }
}

pub struct ImageRemoteCircleHandler {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub url: String,
}

impl IHandler for ImageRemoteCircleHandler {
    fn draw(&self, carrier: &mut RgbaImage) {
        let origin_img = get_remote_resource(&self.url).unwrap();

        let painter = core::resize_image(origin_img, self.width, self.height, imageops::Nearest);

        let circle_img = image_to_circle(painter.into());

        imageops::overlay(carrier, &circle_img, self.x as i64, self.y as i64);
    }
}
