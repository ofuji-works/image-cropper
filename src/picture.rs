use image::{DynamicImage, ImageOutputFormat};

pub struct Picture {
    pub data: DynamicImage,
}

pub fn get_format(format: &str) -> ImageOutputFormat {
    let fmt = match format {
        "png" => ImageOutputFormat::Png,
        "jpg" => ImageOutputFormat::Jpeg(80),
        "bmp" => ImageOutputFormat::Bmp,
        "gif" => ImageOutputFormat::Gif,
        unsupport => ImageOutputFormat::Unsupported(String::from(unsupport)),
    };

    fmt
}
