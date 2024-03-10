use image::{ImageBuffer, Rgb};

use crate::core::core::{load_font, text_to_image};

use super::canvash_handler::IHandler;

pub struct TextHandler {
    pub title: String,
    pub x: i32,
    pub y: i32,
    pub color: Rgb<u8>,
    pub fontsize: f32,
    pub font: &'static [u8],
}

impl IHandler for TextHandler {
    fn draw(&self, carrier: &mut ImageBuffer<Rgb<u8>, Vec<u8>>) {
        println!("{}", "got here text merge");
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
