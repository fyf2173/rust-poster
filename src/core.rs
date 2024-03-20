mod core;

pub use core::*;

#[cfg(test)]
mod tests {
    use image::{Rgb, RgbImage, Rgba, RgbaImage};

    use super::*;

    #[test]
    fn it_merge_image_to_buffer() {
        let mut img = RgbaImage::new(400, 400);
        let painter = image::open("asset/R-C_320.png").unwrap();
        core::meger_image_to_buffer(&mut img, &painter, 0, 0)
            .save("./asset/test_buffer_result.png")
            .unwrap();
    }

    #[test]
    fn it_text_to_image() {
        let mut img = RgbImage::new(200, 200);
        core::text_to_image(
            &mut img,
            "起来，起来，不想上班的人们！！！",
            0,
            0,
            Rgb([255, 255, 255]),
            16.0,
            core::load_font(include_bytes!("../asset/msyhbd.ttf")),
        )
        .save("./asset/test_text_to_image_result.png")
        .unwrap();
    }

    #[test]
    fn it_text_to_merge_image() {
        let mut img = RgbaImage::new(400, 400);
        let painter = image::open("asset/R-C_320.png").unwrap();
        // 透明底不能写上字，得非透明底的图片
        core::meger_image_to_buffer(&mut img, &painter, 0, 0);
        core::text_to_image(
            &mut img,
            "起来，起来，不想上班的人们！！！",
            0,
            0,
            Rgba([255, 255, 255, 255]),
            16.0 * 2.0,
            core::load_font(include_bytes!("../asset/msyhbd.ttf")),
        )
        .save("./asset/text_to_merge_image_result.png")
        .unwrap();
    }
}
