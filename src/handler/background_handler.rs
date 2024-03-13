use image::{imageops, RgbaImage};

use crate::core::core::{get_remote_resource, resize_image};

use super::canvas_handler::IHandler;

pub struct BackgroundHandler {
    pub path: String,
}

impl IHandler for BackgroundHandler {
    fn draw(&self, carrier: &mut RgbaImage) {
        let bg = image::open(&self.path).unwrap();
        // println!("{}", "got here bg merge");
        // meger_image_to_buffer(
        //     carrier,
        //     // &bg.resize(carrier.width(), carrier.height(), imageops::Nearest)
        //     &resize_image(bg, carrier.width(), carrier.height(), imageops::Nearest),
        //     0,
        //     0,
        // );
        imageops::overlay(
            carrier,
            &resize_image(bg, carrier.width(), carrier.height(), imageops::Nearest),
            0,
            0,
        );
    }
}

pub struct BackgroundRemoteHandler {
    pub url: String,
}

impl IHandler for BackgroundRemoteHandler {
    fn draw(&self, carrier: &mut RgbaImage) {
        let bg = get_remote_resource(&self.url).unwrap();
        imageops::overlay(
            carrier,
            &resize_image(bg, carrier.width(), carrier.height(), imageops::Nearest),
            0,
            0,
        );
        // meger_image_to_buffer(
        //     carrier,
        //     &resize_image(bg, carrier.width(), carrier.height(), imageops::Nearest),
        //     0,
        //     0,
        // );
    }
}
