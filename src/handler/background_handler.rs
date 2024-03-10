use image::{imageops, ImageBuffer, Rgb};

use crate::core::core::{get_remote_resource, meger_image_to_buffer, resize_image};

use super::canvash_handler::IHandler;

pub struct BackgroundHandler {
    pub path: String,
}

impl IHandler for BackgroundHandler {
    fn draw(&self, carrier: &mut ImageBuffer<Rgb<u8>, Vec<u8>>) {
        let bg = image::open(&self.path).unwrap();
        println!("{}", "got here bg merge");
        meger_image_to_buffer(
            carrier,
            // &bg.resize(carrier.width(), carrier.height(), imageops::Nearest)
            &resize_image(bg, carrier.width(), carrier.height(), imageops::Nearest).into_rgb8(),
            0,
            0,
        );
    }
}

pub struct BackgroundRemoteHandler {
    pub url: String,
}

impl IHandler for BackgroundRemoteHandler {
    fn draw(&self, carrier: &mut ImageBuffer<Rgb<u8>, Vec<u8>>) {
        let bg = get_remote_resource(&self.url).unwrap();
        meger_image_to_buffer(
            carrier,
            &resize_image(bg, carrier.width(), carrier.height(), imageops::Nearest).into_rgb8(),
            0,
            0,
        );
    }
}
