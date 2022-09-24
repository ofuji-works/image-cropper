extern crate console_error_panic_hook;

use wasm_bindgen::prelude::*;

use image::*;
use js_sys::*;
use std::io::Cursor;

#[wasm_bindgen]
extern "C" {
    pub fn time(s: &str);
    #[wasm_bindgen(js_namespace = console)]

    #[wasm_bindgen(js_namespace = console)]
    pub fn timeEnd(s: &str);
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
    let img = load_from_memory(&buffer).expect("Error occurs at load image from buffer.");

    // 書き込み先のデータの定義
    let width = (end_x - start_x) as u32;
    let height = (end_y - start_y) as u32;
    let mut new_image = ImageBuffer::new(width, height);

    let mut index_x = 0;
    let mut index_y = 0;
    for x in start_x..end_x {
        for y in start_y..end_y {
            let pix  = img.get_pixel(x as u32, y as u32);
            new_image.put_pixel(index_x, index_y, pix);
            index_y += 1;
        }
        index_x += 1;
    }

    let fmt = match format  {
        "png" => ImageOutputFormat::Png,
        "jpg" => ImageOutputFormat::Jpeg(80),
        "bmp" => ImageOutputFormat::Bmp,
        "gif" => ImageOutputFormat::Gif,
        unsupport => ImageOutputFormat::Unsupported(String::from(unsupport))
    };
    let mut vec: Vec<u8> = Vec::new();
    new_image.write_to(&mut Cursor::new(&mut vec), fmt).expect("Error");

    Uint8Array::new(&unsafe { Uint8Array::view(&vec) }.into())
}
