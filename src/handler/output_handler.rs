use image::RgbaImage;

pub trait IOutput {
    fn save(&self, carrier: &mut RgbaImage);
}

pub struct OutputLocal {
    pub path: String,
}

impl IOutput for OutputLocal {
    fn save(&self, carrier: &mut RgbaImage) {
        carrier.save(&self.path).unwrap();
    }
}
