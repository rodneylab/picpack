mod fit;
mod image_utilities;

use image::{DynamicImage::ImageRgba8, ImageBuffer, ImageFormat, ImageOutputFormat};
use image_utilities::{image_to_base64, resize_image};
use serde::Serialize;
use std::io::{Cursor, Read, Seek};
use thumbhash::{rgba_to_thumb_hash, thumb_hash_to_rgba};
use wasm_bindgen::{prelude::*, JsValue};

#[derive(Debug, Eq, PartialEq, Serialize)]
struct PlaceholderResult {
    placeholder: Option<String>,
    error: Option<String>,
}

fn error_result(message: &str) -> JsValue {
    let result = PlaceholderResult {
        placeholder: None,
        error: Some(message.into()),
    };
    serde_wasm_bindgen::to_value(&result).unwrap()
}

#[wasm_bindgen]
pub fn image_placeholder(image_bytes: &[u8]) -> JsValue {
    let input_image = match image::load_from_memory(image_bytes) {
        Ok(value) => value,
        Err(_) => return error_result("Unable to read input image bytes."),
    };
    let (thumbhash_width, thumbhash_height, resized_image) =
        resize_image(&input_image, Some(100), None, Some("clip"));

    // create thumbhash image
    let hash = rgba_to_thumb_hash(
        thumbhash_width.try_into().unwrap(),
        thumbhash_height.try_into().unwrap(),
        &resized_image.into_bytes(),
    );
    let (placeholder_width, placeholder_height, placeholder_bytes) =
        thumb_hash_to_rgba(&hash).unwrap();
    let placeholder_image = ImageRgba8(
        ImageBuffer::from_raw(
            placeholder_width.try_into().unwrap(),
            placeholder_height.try_into().unwrap(),
            placeholder_bytes,
        )
        .unwrap(),
    );

    // create base64 image representation
    let format = image::guess_format(image_bytes);
    let mut cursor = Cursor::new(Vec::new());
    match format {
        Ok(ImageFormat::Png) => {
            placeholder_image
                .write_to(&mut cursor, ImageOutputFormat::Png)
                .unwrap();
        }
        Ok(ImageFormat::Jpeg) => {
            let jpeg_quality = 90;
            placeholder_image
                .write_to(&mut cursor, ImageOutputFormat::Jpeg(jpeg_quality))
                .unwrap();
        }
        Ok(_) | Err(_) => {
            return error_result("Image format not currently supported.");
        }
    };
    let mut buffer = Vec::new();
    cursor.rewind().unwrap();
    cursor.read_to_end(&mut buffer).unwrap();

    // generate base64
    let format = format.expect("Expected supported image format");
    let placeholder = match image_to_base64(&buffer, format) {
        Some(value) => value,
        None => return error_result("Error generating image base64"),
    };

    let result = PlaceholderResult {
        placeholder: Some(placeholder),
        error: None,
    };
    serde_wasm_bindgen::to_value(&result).unwrap()
}
