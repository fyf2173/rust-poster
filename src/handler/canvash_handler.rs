use image::{ImageBuffer, Rgb, RgbImage};

use super::output_handler::IOutput;

pub trait IHandler {
    fn draw(&self, carrier: &mut ImageBuffer<Rgb<u8>, Vec<u8>>);
}

pub struct CanvasHandler {
    pub x: u32,
    pub y: u32,
    pub output: Box<dyn IOutput>,
    pub background: Box<dyn IHandler>,
    pub elements: Vec<Box<dyn IHandler>>,
}

impl CanvasHandler {
    pub fn run(&self) {
        let mut carrier = RgbImage::new(self.x, self.y);
        self.background.as_ref().draw(&mut carrier); // 先画背景图

        for elem in self.elements.iter() {
            elem.draw(&mut carrier)
        }
        let _ = &self.output.as_ref().save(&mut carrier);
    }
}
