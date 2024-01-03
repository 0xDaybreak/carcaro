use std::io::Read;
use crate::handle_errors::Error;
use image::{DynamicImage, GenericImageView, Rgba};
use rayon::iter::ParallelIterator;

pub async fn color_swap(
    base_urls: Vec<String>,
    target_color: [u8; 3],
    mask_urls: Vec<String>,
) -> Result<(), Error> {
    println!("Started mask and models extraction");
    extract_mask_and_model(base_urls, "base").await.expect("TODO: panic message");
    extract_mask_and_model(mask_urls, "mask").await.expect("TODO: panic message");
    println!("Extracted mask and models");
    apply_color_shift(target_color).await.expect("TODO: panic message");
    println!("applied hue shift");
    Ok(())
}

async fn extract_mask_and_model(
    urls: Vec<String>,
    dir: &str,
) -> Result<(), Error> {
    let mut tasks = Vec::new();
    let dir_arc = std::sync::Arc::new(dir.to_string());


    for (i, u) in urls.iter().enumerate() {
        let task = tokio::spawn(prepare_images(i, u.clone(), dir_arc.clone()));
        tasks.push(task)
    }
    for task in tasks {
        task.await.unwrap().expect("TODO: panic message");
    }
    Ok(())
}


async fn prepare_images(i: usize, url: String, dir: std::sync::Arc<String>) -> Result<(), Error> {
    let response = reqwest::get(url).await.unwrap();
    if !response.status().is_success() {
        return Err(Error::ColorSwapError);
    }
    let img_bytes = response.bytes().await.unwrap();

    let image = image::load_from_memory(&img_bytes).unwrap();
    let filename = format!("src/{}/saved_{}.png", dir, i);
    image.save(filename).expect("failed to save");
    Ok(())
}


pub async fn apply_color_shift(target_color: [u8; 3]) -> Result<(), Error> {
    for i in 0..=11 {
        let base_input = format!("src/base/saved_{}.png", i);
        let mask_input = format!("src/mask/saved_{}.png", i);
        let output = format!("src/base/saved_{}.png", i);
        let mut base_image = image::open(&base_input).expect("Failed to open image");
        let mask_image = image::open(&mask_input).expect("Failed to open image");
        colorize_images(&mut base_image, &mask_image, target_color).await;
        println!("colorized images");
        base_image.save(output).expect("Failed to save image");
    }
    Ok(())
}

async fn colorize_images(
    base_image: &mut DynamicImage,
    mask_image: &DynamicImage,
    target_color: [u8; 3],
) {
    let mut rgba_base_image = base_image.to_rgba8();
    let mut rgba_mask_image = mask_image.to_rgba8();

    for (base_pixel, mask_pixel) in rgba_base_image.pixels_mut().zip(rgba_mask_image.pixels()) {
        if mask_pixel.0[3] != 0 {
            let colorized_pixel = adjust_color(base_pixel, target_color);
            *base_pixel = colorized_pixel;
        }
    }

    *base_image = DynamicImage::ImageRgba8(rgba_base_image);
}

fn adjust_color(base_pixel: &Rgba<u8>, target_color: [u8; 3]) -> Rgba<u8> {
    let (mut h, s, v) = rgb_to_hsv(base_pixel[0], base_pixel[1], base_pixel[2]);
    let (r, g, b) = hsv_to_rgb(hue_from_rgb(target_color[0], target_color[1], target_color[2]), s, v);
    Rgba([r, g, b, base_pixel[3]])
}

fn hue_from_rgb(r: u8, g: u8, b: u8) -> u16 {
    let (h, _, _) = rgb_to_hsv(r, g, b);
    h
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