pub mod background_handler;
pub mod canvash_handler;
pub mod output_handler;
pub mod text_handler;

mod tests {

    use image::Rgb;

    use super::*;

    #[test]
    fn it_run_handler() {
        let canvas = canvash_handler::CanvasHandler {
            x: 300,
            y: 300,
            output: Box::new(output_handler::OutputLocal {
                path: String::from("./asset/handler_test_2.png"),
            }),
            background: Box::new(background_handler::BackgroundHandler {
                path: "./asset/R-C_320.png".to_string(),
            }),
            elements: vec![Box::new(text_handler::TextHandler {
                title: "hello world hahah".to_string(),
                x: 0,
                y: 0,
                color: Rgb([0, 0, 0]),
                fontsize: 16.0,
                font: include_bytes!("../asset/msyhbd.ttf"),
            })],
        };
        canvas.run();
    }

    #[test]
    fn it_run_remote_handler() {
        let canvas = canvash_handler::CanvasHandler {
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
                    color: Rgb([0, 0, 0]),
                    fontsize: 16.0,
                    font: include_bytes!("../asset/msyhbd.ttf"),
                }),
            ],
        };
        canvas.run();
    }
}
