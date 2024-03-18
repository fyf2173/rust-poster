use image::{imageops, Rgba, RgbaImage};
use qrcode::QrCode;

use super::canvas_handler::IHandler;

pub struct QrcodeHandler {
    pub x: i64,
    pub y: i64,
    pub witdh: u32,
    pub height: u32,
    pub content: String,
    pub light_color: Rgba<u8>,
    pub dark_color: Rgba<u8>,
    pub quiet_zone: bool,
}

impl IHandler for QrcodeHandler {
    fn draw(&self, carrier: &mut RgbaImage) {
        let qrcode_img = QrCode::new(&self.content)
            .unwrap()
            .render()
            .dark_color(self.dark_color)
            .light_color(self.light_color) // adjust colors
            .quiet_zone(self.quiet_zone) // disable quiet zone (white border)
            .min_dimensions(self.witdh, self.height) // sets minimum image size
            .build();

        imageops::overlay(carrier, &qrcode_img, self.x, self.y);
    }
}
