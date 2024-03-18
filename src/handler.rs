pub mod background_handler;
pub mod canvas_handler;
pub mod imagecircle_handler;
pub mod line_handler;
pub mod output_handler;
pub mod qrcode_handler;
pub mod rectangle_handler;
pub mod text_handler;

mod tests {

    use image::Rgba;

    use super::*;

    #[test]
    fn it_run_handler() {
        let canvas = canvas_handler::CanvasHandler {
            x: 300,
            y: 300,
            output: Box::new(output_handler::OutputLocal {
                path: String::from("./asset/handler_test_2.png"),
            }),
            background: Box::new(background_handler::BackgroundHandler {
                path: "./asset/R-C_320.png".to_string(),
            }),
            elements: vec![
                Box::new(text_handler::TextHandler {
                    title: "hello world hahah".to_string(),
                    x: 0,
                    y: 0,
                    color: Rgba([0, 0, 0, 255]),
                    fontsize: 16.0,
                    font: include_bytes!("../asset/msyhbd.ttf"),
                }),
                Box::new(rectangle_handler::RectangleHandler {
                    x: 30,
                    y: 30,
                    with: 100,
                    height: 10,
                    color: Rgba([0, 0, 0, 0]),
                }),
                Box::new(rectangle_handler::RectangleHandler {
                    x: 40,
                    y: 40,
                    with: 100,
                    height: 2,
                    color: Rgba([0, 0, 0, 0]),
                }),
                Box::new(line_handler::LineHandler {
                    x1: 150f32,
                    y1: 150f32,
                    x2: 250f32,
                    y2: 150f32,
                    color: Rgba([0, 0, 0, 255]),
                }),
                // Box::new(imagecircle_handler::ImageCircleHandler {
                //     path: String::from("./asset/R-C_320.png"),
                //     x: 100,
                //     y: 100,
                //     width: 100,
                //     height: 100,
                // }),
                Box::new(imagecircle_handler::ImageRemoteCircleHandler {
                    x: 150,
                    y: 150,
                    width: 50,
                    height: 50,
                    url: String::from("https://img-crs-test.vchangyi.com/upload/265379470454976/qrcode1709273820.png"),
                }),
                Box::new(qrcode_handler::QrcodeHandler {
                    x: 50,
                    y: 50,
                    witdh: 50,
                    height: 50,
                    light_color: Rgba([224, 224, 224, 255]),
                    dark_color: Rgba([0, 0, 128, 255]),
                    quiet_zone: false,
                    content: String::from("Hello World"),
                }),
            ],
        };
        canvas.run_thread();
    }

    #[test]
    fn it_run_remote_handler() {
        let canvas = canvas_handler::CanvasHandler {
            x: 300,
            y: 300,
            output: Box::new(output_handler::OutputLocal{
                path: String::from("./asset/handler_remote_test_2.png"),
            }),
            background: Box::new(background_handler::BackgroundRemoteHandler {
                url: String::from("https://img-crs-test.vchangyi.com/uploads/2024/2/19/a06ea01d7b71e205094d7241deb66ebc.png") ,
               }),
            elements: vec![
                Box::new(text_handler::TextHandler {
                    title: "hello world".to_string(),
                    x: 0,
                    y: 0,
                    color: Rgba([0, 0, 0, 255]),
                    fontsize: 16.0,
                    font: include_bytes!("../asset/msyhbd.ttf"),
                }),
            ],
        };
        canvas.run();
    }
}
