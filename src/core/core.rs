use image::{
    imageops, DynamicImage, GenericImage, GenericImageView, ImageBuffer, Pixel, Primitive, Rgb,
    RgbImage,
};
use imageproc::drawing::draw_text_mut;
use reqwest::Error;
use rusttype::{Font, Scale};

// new_canvas 实例化画布
pub fn new_canvas(width: u32, height: u32) -> RgbImage {
    RgbImage::new(width, height)
}

// meger_image_to_buffer 合并图片
pub fn meger_image_to_buffer<'a, I, P, S>(
    carrier: &'a mut ImageBuffer<P, Vec<S>>,
    painter: &I,
    x: u32,
    y: u32,
) -> &'a mut ImageBuffer<P, Vec<S>>
where
    I: GenericImageView<Pixel = P>,
    P: Pixel<Subpixel = S> + 'static,
    S: Primitive + 'static,
{
    // Copy input image at the correct location in the buffer image.
    let _ = carrier.copy_from(painter, x, y);

    carrier
}

pub fn resize_image(
    img: DynamicImage,
    width: u32,
    height: u32,
    filter: imageops::FilterType,
) -> DynamicImage {
    img.resize(width, height, filter)
}

// load_font 加载字体，font_data = include_bytes!("字体路径")
pub fn load_font(font_data: &[u8]) -> Vec<u8> {
    Vec::from(font_data)
}

// text_to_image 文字写入图片
pub fn text_to_image<'a>(
    carrier: &'a mut ImageBuffer<Rgb<u8>, Vec<u8>>,
    title: &'a str,
    x: i32,
    y: i32,
    color: Rgb<u8>,
    fontsize: f32,
    font: Vec<u8>,
) -> &'a mut ImageBuffer<Rgb<u8>, Vec<u8>> {
    let font = Font::try_from_vec(font).unwrap();

    draw_text_mut(
        carrier,
        color,
        x,
        y,
        Scale {
            x: fontsize,
            y: fontsize,
        },
        &font,
        &title,
    );

    carrier
}

// get_remote_resource 获取远程图片
pub fn get_remote_resource(link: &str) -> Result<DynamicImage, Error> {
    let image_vec = reqwest::blocking::get(link)?.bytes()?.to_vec();
    Ok(image::load_from_memory(image_vec.as_slice()).unwrap())
}
