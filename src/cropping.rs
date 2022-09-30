extern crate console_error_panic_hook;

use wasm_bindgen::prelude::*;

use crate::picture::{get_format, Picture};
use image::{imageops, load_from_memory};
use js_sys::Uint8Array;
use std::io::Cursor;

struct CroppingFrame {
    start_x: u32,
    start_y: u32,
    width: u32,
    height: u32,
}

impl CroppingFrame {
    fn crop(&self, img: &mut Picture, format: &str) -> Uint8Array {
        let cropepd_img = imageops::crop(
            &mut img.data,
            self.start_x,
            self.start_y,
            self.width,
            self.height,
        );

        let image_buffer = cropepd_img.to_image();
        let mut vec = Vec::new();

        image_buffer
            .write_to(&mut Cursor::new(&mut vec), get_format(format))
            .expect("Error");

        Uint8Array::new(&unsafe { Uint8Array::view(&vec) }.into())
    }
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

    let mut picture = Picture { data: img };

    // 書き込み先のデータの定義
    let width = (end_x - start_x) as u32;
    let height = (end_y - start_y) as u32;
    let cropping_frame = CroppingFrame {
        start_x: start_x as u32,
        start_y: start_y as u32,
        width,
        height,
    };

    cropping_frame.crop(&mut picture, format)
}
