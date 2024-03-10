use image::{ImageBuffer, Rgb};

pub trait IOutput {
    fn save(&self, carrier: &mut ImageBuffer<Rgb<u8>, Vec<u8>>);
}

pub struct OutputLocal {
    pub path: String,
}

impl IOutput for OutputLocal {
    fn save(&self, carrier: &mut ImageBuffer<Rgb<u8>, Vec<u8>>) {
        carrier.save(&self.path).unwrap();
    }
}
