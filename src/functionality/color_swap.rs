use std::io::Read;
use crate::handle_errors::Error;
use image::{DynamicImage, GenericImageView, Rgba, RgbaImage};
use crate::types::image::NewImage;
use azure_core::error::{ErrorKind, ResultExt};
use azure_storage::prelude::*;
use azure_storage_blobs::prelude::*;
use futures::stream::StreamExt;
use crate::functionality::container_generation;

pub async fn color_swap(
    base_urls: Vec<String>,
    target_color: [i32; 3],
    mask_urls: Vec<String>,
) -> Result<(), Error> {
    extract_mask_and_model(base_urls, "base").await.expect("TODO: panic message");
    extract_mask_and_model(mask_urls, "mask").await.expect("TODO: panic message");
    apply_hue_shift().await;
    Ok(())
}

pub async fn extract_mask_and_model(
    urls: Vec<String>,
    dir: &str,
) -> Result<(), Error> {
    for (i, u) in urls.iter().enumerate() {
        let response = reqwest::get(u).await.unwrap();
        if !response.status().is_success() {
            return Err(Error::ColorSwapError);
        }
        let img_bytes = response.bytes().await.unwrap();

        let image = image::load_from_memory(&img_bytes).unwrap();
        let filename = format!("src/{}/saved_{}.png", dir, i);
        image.save(filename).expect("failed to save");
    }
    Ok(())
}


pub async fn apply_hue_shift() -> Result<(), Error> {
    let hue_adjustment = 180;
    for i in 0..=11 {
        let base_input = format!("src/base/saved_{}.png", i);
        let mask_input = format!("src/mask/saved_{}.png", i);
        let output = format!("src/base/saved_{}.png", i);
        let mut base_image = image::open(&base_input).expect("Failed to open image");
        let mut mask_image = image::open(&mask_input).expect("Failed to open image");
        shift_colors(&mut mask_image, hue_adjustment).await;
        overlay_images(&mut base_image, &mask_image);
        base_image.save(output).expect("Failed to save image");
    }
    Ok(())
}


pub async fn shift_colors(
    image: &mut DynamicImage,
    hue_shift: i16,
) {
    let rgba_image = image.to_rgba8();
    let (width, height) = rgba_image.dimensions();
    let mut transformed_image = RgbaImage::new(width, height);
    for y in 0..height {
        for x in 0..width {
            let pixel = rgba_image.get_pixel(x, y);
            let transformed_pixel = adjust_hue(pixel, hue_shift);
            transformed_image.put_pixel(x, y, transformed_pixel);
        }
    }
    *image = DynamicImage::ImageRgba8(transformed_image);
}


fn overlay_images(base_image: &mut DynamicImage, mask_image: &DynamicImage) {
    let (base_width, base_height) = base_image.dimensions();
    let (mask_width, mask_height) = mask_image.dimensions();
    let resized_mask = image::imageops::resize(mask_image, base_width, base_height, image::imageops::FilterType::Nearest);
    let mut cloned_base = base_image.clone();
    image::imageops::overlay(&mut cloned_base, &resized_mask, 0, 0);
    *base_image = cloned_base;
}


fn adjust_hue(pixel: &Rgba<u8>, hue_adjustment: i16) -> Rgba<u8> {
    let (mut h, s, v) = rgb_to_hsv(pixel[0], pixel[1], pixel[2]);
    h = ((h as i16 + hue_adjustment + 360) % 360) as u16;
    let (r, g, b) = hsv_to_rgb(h, s, v);
    Rgba([r, g, b, pixel[3]])
}

fn rgb_to_hsv(r: u8, g: u8, b: u8) -> (u16, f32, f32) {
    let r = r as f32 / 255.0;
    let g = g as f32 / 255.0;
    let b = b as f32 / 255.0;

    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let delta = max - min;

    let hue = if delta.abs() < f32::EPSILON {
        0.0
    } else if max == r {
        60.0 * ((g - b) / delta % 6.0)
    } else if max == g {
        60.0 * ((b - r) / delta + 2.0)
    } else {
        60.0 * ((r - g) / delta + 4.0)
    };

    let saturation = if max.abs() < f32::EPSILON {
        0.0
    } else {
        delta / max
    };

    let value = max;

    (hue as u16, saturation, value)
}

fn hsv_to_rgb(h: u16, s: f32, v: f32) -> (u8, u8, u8) {
    let c = v * s;
    let x = c * (1.0 - ((h as f32 / 60.0) % 2.0 - 1.0).abs());
    let m = v - c;

    let (r, g, b) = if h < 60 {
        (c, x, 0.0)
    } else if h < 120 {
        (x, c, 0.0)
    } else if h < 180 {
        (0.0, c, x)
    } else if h < 240 {
        (0.0, x, c)
    } else if h < 300 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };

    (
        ((r + m) * 255.0).round() as u8,
        ((g + m) * 255.0).round() as u8,
        ((b + m) * 255.0).round() as u8,
    )
}
