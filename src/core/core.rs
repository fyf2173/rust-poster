use conv::ValueInto;
use image::{
    imageops, DynamicImage, GenericImage, GenericImageView, ImageBuffer, Pixel, Primitive,
    RgbImage, Rgba, RgbaImage,
};
use imageproc::{
    definitions::Clamp,
    drawing::{draw_text_mut, Canvas},
};
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
pub fn text_to_image<'a, C>(
    carrier: &'a mut C,
    title: &'a str,
    x: i32,
    y: i32,
    color: C::Pixel,
    fontsize: f32,
    font: Vec<u8>,
) -> &'a mut C
where
    C: Canvas,
    <C::Pixel as Pixel>::Subpixel: ValueInto<f32> + Clamp<f32>,
{
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

// image_to_circle 圆形图片
pub fn image_to_circle(img: RgbaImage) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let (width, height) = img.dimensions();

    let mut imgbuf = RgbaImage::new(width, height);

    let radius = width.min(height) / 2;
    let center = (width / 2, height / 2);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let dx = center.0 as i32 - x as i32;
        let dy = center.1 as i32 - y as i32;
        if (dx * dx + dy * dy) < (radius as i32 * radius as i32) {
            *pixel = *img.get_pixel(x, y);
        } else {
            *pixel = image::Rgba([0, 0, 0, 0]);
        }
    }

    imgbuf
}
