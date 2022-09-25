extern crate console_error_panic_hook;

use wasm_bindgen::prelude::*;

use image::*;
use js_sys::*;
use std::io::Cursor;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn time(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    pub fn timeEnd(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

#[wasm_bindgen]
pub fn crop_image(
    array_buffer: Uint8Array,
    start_x: usize,
    start_y: usize,
    end_x: usize,
    end_y: usize,
    format: &str,
) -> Uint8Array {
    console_error_panic_hook::set_once();
    let buffer = array_buffer.to_vec();
    let mut img = load_from_memory(&buffer).expect("Error occurs at load image from buffer.");

    // 書き込み先のデータの定義
    let width = (end_x - start_x) as u32;
    let height = (end_y - start_y) as u32;

    let cropepd_img = imageops::crop(&mut img, start_x as u32, start_y as u32, width, height);
    let image_buffer = cropepd_img.to_image();
    let fmt = match format {
        "png" => ImageOutputFormat::Png,
        "jpg" => ImageOutputFormat::Jpeg(80),
        "bmp" => ImageOutputFormat::Bmp,
        "gif" => ImageOutputFormat::Gif,
        unsupport => ImageOutputFormat::Unsupported(String::from(unsupport)),
    };
    let mut vec = Vec::new();
    image_buffer
        .write_to(&mut Cursor::new(&mut vec), fmt)
        .expect("Error");

    Uint8Array::new(&unsafe { Uint8Array::view(&vec) }.into())
}
