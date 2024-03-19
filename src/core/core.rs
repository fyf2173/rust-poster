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

/// 实例化画布
///
/// # Examples
///
/// ```
/// let canvas = new_canvas(300,400);
/// ```
pub fn new_canvas(width: u32, height: u32) -> RgbImage {
    RgbImage::new(width, height)
}

/// 合并图片到承载的图床上
///
/// # Examples
///
/// ```
/// let mut img = RgbaImage::new(400, 400);
/// let painter = image::open("asset/R-C_320.png").unwrap();
/// core::meger_image_to_buffer(&mut img, &painter, 0, 0)
///     .save("./asset/test_buffer_result.png")
///     .unwrap();
/// ```
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

/// 修改图片的尺寸
pub fn resize_image(
    img: DynamicImage,
    width: u32,
    height: u32,
    filter: imageops::FilterType,
) -> DynamicImage {
    img.resize(width, height, filter)
}

/// 加载字体
///
/// # Examples
///
/// ```
/// let font_data = include_bytes!("字体路径");
/// let fontBody = load_font(font_data);
/// ```
pub fn load_font(font_data: &[u8]) -> Vec<u8> {
    Vec::from(font_data)
}

/// 文字写入图片
///
/// # Examples
///
/// ```
/// let mut img = RgbaImage::new(400, 400);
/// let painter = image::open("asset/R-C_320.png").unwrap();
/// // 透明底不能写上字，得非透明底的图片
/// core::meger_image_to_buffer(&mut img, &painter, 0, 0);
/// core::text_to_image(
///     &mut img,
///     "起来，起来，不想上班的人们！！！",
///     0,
///     0,
///     Rgba([255, 255, 255, 255]),
///     16.0 * 2.0,
///     core::load_font(include_bytes!("../asset/msyhbd.ttf")),
/// )
/// .save("./asset/text_to_merge_image_result.png")
/// .unwrap();
/// ```
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

/// 获取远程图片
///
/// # Examples
///
/// ```
/// let bg = get_remote_resource("http://www.test.com/example.png".to_string()).unwrap();
/// ```
pub fn get_remote_resource(link: &str) -> Result<DynamicImage, Error> {
    let image_vec = reqwest::blocking::get(link)?.bytes()?.to_vec();
    Ok(image::load_from_memory(image_vec.as_slice()).unwrap())
}

/// 将RGBA图片转为圆形图片
///
/// # Examples
///
/// ```
/// let painter = core::resize_image(origin_img, 100, 100, imageops::Nearest);
/// let circle_img = image_to_circle(painter.into());
/// ```
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
