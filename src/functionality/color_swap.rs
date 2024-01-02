use std::io::Read;
use colorsys::{Hsl};
use crate::handle_errors::Error;
use image::{DynamicImage, GenericImageView, Rgba};

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
        base_image.to_rgba8().save(output).expect("Failed to save image");
    }
    Ok(())
}

async fn colorize_images(
    base_image: &mut DynamicImage,
    mask_image: &DynamicImage,
    target_color: [u8; 3],
) -> Result<(), Error> {
    let mut rgba_base_image = base_image.to_rgba32f();
    let rgba_mask_image = mask_image.to_rgba8();

    for (pixel, mask_pixel) in rgba_base_image.pixels_mut().zip(rgba_mask_image.pixels()) {
        if mask_pixel[0] != 0 || mask_pixel[1] != 0 || mask_pixel[2] != 0 {
            *pixel = adjust_color(&pixel, target_color);
        }
    }

    *base_image = DynamicImage::ImageRgba32F(rgba_base_image);
    Ok(())
}

fn adjust_color(base_pixel: &Rgba<f32>, target_color: [u8; 3]) -> Rgba<f32> {
    let r = base_pixel.0[0] as f64;
    let g = base_pixel.0[1] as f64;
    let b = base_pixel.0[2] as f64;

    let target_r = target_color[0] as f64;
    let target_g = target_color[1] as f64;
    let target_b = target_color[2] as f64;

    let target_rgb = colorsys::Rgb::new(target_r, target_g, target_b, None);
    let target_hue = Hsl::from(target_rgb).hue();

    let base_rgb:colorsys::Rgb = colorsys::Rgb::new(r, g, b, None);
    let mut base_hsl = Hsl::from(base_rgb);
    base_hsl.set_hue(0.);
    base_hsl.set_hue(target_hue);
    let base_rgb:colorsys::Rgb = colorsys::Rgb::from(&mut base_hsl);

    Rgba([base_rgb.red() as f32, base_rgb.green() as f32, base_rgb.blue() as f32, base_pixel[3]])
}