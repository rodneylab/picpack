use crate::fit::output_dimensions;
use base64::{Engine as _, engine::general_purpose};
use image::{
    DynamicImage::{self, ImageRgba8},
    ImageFormat,
    imageops::{self, FilterType},
};

fn decimal_to_u8(decimal: f32) -> u8 {
    assert!((0.0..=1.0).contains(&decimal));
    ((256.0 * decimal).round()) as u8
}

pub fn format_to_mime_type(format: ImageFormat) -> Option<String> {
    match format {
        ImageFormat::Png => Some("image/png".to_string()),
        ImageFormat::Jpeg => Some("image/jpeg".to_string()),
        _ => None,
    }
}

pub fn image_dimensions(image: &DynamicImage) -> (u32, u32) {
    let width = image.width();
    let height = image.height();

    (width, height)
}

pub fn image_to_base64(image_bytes: &[u8], mime_type: &str) -> Option<String> {
    let mut base64 = String::new();
    general_purpose::STANDARD_NO_PAD.encode_string(image_bytes, &mut base64);
    let result = format!("data:{mime_type};base64,{base64}");
    Some(result)
}

pub fn resize_image(
    input_image: &DynamicImage,
    input_width: u32,
    input_height: u32,
    output_width: Option<u32>,
    output_height: Option<u32>,
    fit: Option<&str>,
) -> (u32, u32, DynamicImage) {
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
