mod fit;
mod image_utilities;

use image::{DynamicImage::ImageRgba8, ImageBuffer, ImageFormat, ImageOutputFormat};
use image_utilities::{
    format_to_mime_type, image_dimensions, image_to_base64, resize_image, rgba_to_hex,
};
use serde::Serialize;
use std::io::{Cursor, Read, Seek};
use thumbhash::{rgba_to_thumb_hash, thumb_hash_to_average_rgba, thumb_hash_to_rgba};
use wasm_bindgen::{prelude::*, JsValue};

#[derive(Debug, Eq, PartialEq, Serialize)]
struct ImageMetadata {
    width: u32,
    height: u32,
    format: String,
}

#[derive(Debug, Eq, PartialEq, Serialize)]
pub struct PlaceholderResult {
    average: Option<String>,
    base64: Option<String>,
    metadata: Option<ImageMetadata>,
    error: Option<String>,
}

fn error_result(message: &str) -> PlaceholderResult {
    PlaceholderResult {
        average: None,
        base64: None,
        metadata: None,
        error: Some(message.into()),
    }
}

pub fn get_image_placeholder(image_bytes: &[u8]) -> PlaceholderResult {
    let input_image = match image::load_from_memory(image_bytes) {
        Ok(value) => value,
        Err(_) => return error_result("Unable to read input image bytes."),
    };
    let (width, height) = image_dimensions(&input_image);
    let (thumbhash_width, thumbhash_height, resized_image) = resize_image(
        &input_image,
        width,
        height,
        Some(100),
        Some(100),
        Some("clip"),
    );

    // create thumbhash image
    let thumbhash_vec = rgba_to_thumb_hash(
        thumbhash_width.try_into().unwrap(),
        thumbhash_height.try_into().unwrap(),
        &resized_image.into_bytes(),
    );
    let average = match thumb_hash_to_average_rgba(&thumbhash_vec) {
        Ok(value) => rgba_to_hex(value),
        Err(_) => return error_result("Unable to determine image average rgba hex value"),
    };
    let (placeholder_width, placeholder_height, placeholder_bytes) =
        thumb_hash_to_rgba(&thumbhash_vec).unwrap();
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
    let mime_type = format_to_mime_type(format).expect("Expected supported image format");

    match image_to_base64(&buffer, &mime_type) {
        Some(placeholder) => PlaceholderResult {
            average: Some(average),
            base64: Some(placeholder),
            metadata: Some(ImageMetadata {
                width,
                height,
                format: mime_type,
            }),
            error: None,
        },
        None => PlaceholderResult {
            average: None,
            base64: None,
            metadata: None,
            error: Some("Error generating image base64".to_string()),
        },
    }
}

/// Generates a base64 image placeholder using the thumbhash algorithm.
/// Also returns an rgba hex code for the image average colour and input image width, height and MIME
/// type metadata.
/// Currently expects image to be in JPEG or PNG format.
#[wasm_bindgen]
pub fn image_placeholder(image_bytes: &[u8]) -> JsValue {
    let result = get_image_placeholder(image_bytes);

    serde_wasm_bindgen::to_value(&result).unwrap()
}

#[cfg(test)]
mod tests {
    use super::{get_image_placeholder, ImageMetadata, PlaceholderResult};
    use std::{fs::File, io::Read, path::Path};

    #[test]
    fn get_image_placeholder_generates_expected_result() {
        // prepare
        let image_path = Path::new("./images/field.jpg");
        let mut image_file = File::open(&image_path).expect("Error opening image file for test");
        let mut image_bytes = Vec::new();
        image_file
            .read_to_end(&mut image_bytes)
            .expect("Error reading image file into vector");

        // act
        let result = get_image_placeholder(image_bytes.as_ref());

        // assert
        let expected_base64 = "data:image/jpeg;base64,/9j/4AAQSkZJRgABAgAAAQABAAD/wAARCAAXACADAREAAhEBAxEB/9sAQwADAgIDAgIDAwMDBAMDBAUIBQUEBAUKBwcGCAwKDAwLCgsLDQ4SEA0OEQ4LCxAWEBETFBUVFQwPFxgWFBgSFBUU/9sAQwEDBAQFBAUJBQUJFA0LDRQUFBQUFBQUFBQUFBQUFBQUFBQUFBQUFBQUFBQUFBQUFBQUFBQUFBQUFBQUFBQUFBQU/8QAHwAAAQUBAQEBAQEAAAAAAAAAAAECAwQFBgcICQoL/8QAtRAAAgEDAwIEAwUFBAQAAAF9AQIDAAQRBRIhMUEGE1FhByJxFDKBkaEII0KxwRVS0fAkM2JyggkKFhcYGRolJicoKSo0NTY3ODk6Q0RFRkdISUpTVFVWV1hZWmNkZWZnaGlqc3R1dnd4eXqDhIWGh4iJipKTlJWWl5iZmqKjpKWmp6ipqrKztLW2t7i5usLDxMXGx8jJytLT1NXW19jZ2uHi4+Tl5ufo6erx8vP09fb3+Pn6/8QAHwEAAwEBAQEBAQEBAQAAAAAAAAECAwQFBgcICQoL/8QAtREAAgECBAQDBAcFBAQAAQJ3AAECAxEEBSExBhJBUQdhcRMiMoEIFEKRobHBCSMzUvAVYnLRChYkNOEl8RcYGRomJygpKjU2Nzg5OkNERUZHSElKU1RVVldYWVpjZGVmZ2hpanN0dXZ3eHl6goOEhYaHiImKkpOUlZaXmJmaoqOkpaanqKmqsrO0tba3uLm6wsPExcbHyMnK0tPU1dbX2Nna4uPk5ebn6Onq8vP09fb3+Pn6/9oADAMBAAIRAxEAPwD1yD4f25XIcfnX0iPJZJN4Sjt4GAYVqiTm7DwrnVcg96GgNrxZoohslGR0rNoaPmbw/wDtW3FwvLn868FY5dzXlOhX9pNp1O563WOQuUpQ/tMQ2d5lnGc1f11dw5Sl4y/aointwFk/WoeNiHKeX6J8LIYU4YV+ZLFyO2xqj4fIiN89dEcXInlPO/E/g8w32Fkxz61r9akLlH2/w1GpW4LyfrSeKkPlP//Z".to_string();
        let PlaceholderResult {
            average,
            base64,
            metadata,
            error,
            ..
        } = result;
        assert_eq!(Some("#6a7774ff".to_string()), average);
        assert_eq!(Some(expected_base64), base64);
        assert_eq!(None, error);

        let ImageMetadata {
            width,
            height,
            format,
        } = metadata.expect("Expected metadata to be some");
        assert_eq!(100, width);
        assert_eq!(75, height);
        assert_eq!("image/jpeg", &format);
    }
}
