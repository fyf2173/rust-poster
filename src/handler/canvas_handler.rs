use std::{
    sync::{Arc, Mutex},
    thread,
};

use image::RgbaImage;

use super::output_handler::IOutput;

pub trait IHandler: Send + Sync {
    fn draw(&self, carrier: &mut RgbaImage);
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
        let mut carrier = RgbaImage::new(self.x, self.y);
        self.background.as_ref().draw(&mut carrier); // 先画背景图

        for elem in self.elements.iter() {
            elem.draw(&mut carrier)
        }
        let _ = &self.output.as_ref().save(&mut carrier);
    }

    pub fn run_thread(&self) {
        let carrier = Arc::new(Mutex::new(RgbaImage::new(self.x, self.y)));
        self.background.as_ref().draw(&mut carrier.lock().unwrap()); // 先画背景图

        thread::scope(|s| {
            for elem in self.elements.iter() {
                let carrier = Arc::clone(&carrier);
                let item = Arc::new(elem);
                s.spawn(move || {
                    item.draw(&mut *carrier.lock().unwrap());
                });
            }
        });

        let _ = &self.output.as_ref().save(&mut carrier.lock().unwrap());
    }
}
