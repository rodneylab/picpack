use crate::fit::output_dimensions;
use base64::{engine::general_purpose, Engine as _};
use image::{
    imageops::{self, FilterType},
    DynamicImage::{self, ImageRgba8},
    ImageFormat,
};

fn decimal_to_u8(decimal: f32) -> u8 {
    assert!((0.0..=1.0).contains(&decimal));
    ((256.0 * decimal).round()) as u8
}

pub fn image_to_base64(image_bytes: &[u8], format: ImageFormat) -> Option<String> {
    let mime_type = match format {
        ImageFormat::Png => "image/png",
        ImageFormat::Jpeg => "image/jpeg",
        _ => return None,
    };

    let mut base64 = String::new();
    general_purpose::STANDARD_NO_PAD.encode_string(image_bytes, &mut base64);
    let result = format!("data:{mime_type};base64,{base64}");
    Some(result)
}

pub fn resize_image(
    input_image: &DynamicImage,
    output_width: Option<u32>,
    output_height: Option<u32>,
    fit: Option<&str>,
) -> (u32, u32, DynamicImage) {
    let input_width = input_image.width();
    let input_height = input_image.height();
    let (resized_width, resized_height) =
        output_dimensions(input_width, input_height, output_width, output_height, fit);
    let image = ImageRgba8(imageops::resize(
        input_image,
        resized_width,
        resized_height,
        FilterType::Lanczos3,
    ));
    (resized_width, resized_height, image)
}

pub fn rgba_to_hex(rgba: (f32, f32, f32, f32)) -> String {
    let (red, green, blue, alpha) = rgba;
    let r = decimal_to_u8(red);
    let g = decimal_to_u8(green);
    let b = decimal_to_u8(blue);
    let a = decimal_to_u8(alpha);
    let result = format!("#{r:x}{g:x}{b:x}{a:x}");

    result
}
