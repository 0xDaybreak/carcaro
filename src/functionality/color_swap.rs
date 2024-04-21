use crate::handle_errors::Error;
use colorsys::{Hsl, Rgb};
use image::{Rgba};
use opencv::{imgcodecs, imgproc};
use opencv::core::{CV_8U, Mat, MatTrait, MatTraitConst, Scalar, Vec3b};

pub async fn color_swap(base_urls: Vec<String>, target_color: [u8; 3]) -> Result<(), Error> {
    println!("Started mask and models extraction");
    extract_mask_and_model(base_urls, "base")
        .await
        .expect("Failed to extract model");
    println!("Extracted mask and models");
    apply_color_shift(target_color)
        .await
        .expect("Failed to apply hue shift");
    println!("applied hue shift");
    Ok(())
}

async fn extract_mask_and_model(urls: Vec<String>, dir: &str) -> Result<(), Error> {
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
    let mut tasks = Vec::new();
    for i in 0..=11 {
        let base_i = format!("src/base/saved_{}.png", i);
        let base_o = format!("src/mask/saved_{}.png", i);

        let task = tokio::spawn(async move {
            //let mut base_image = image::open(&base_io).expect("Failed to open image");
            colorize_images(&base_i, &base_o, target_color)
                .await
                .expect("Failed to colorize images");
            println!("colorized images");
        });
        tasks.push(task);
    }

    for task in tasks {
        task.await.unwrap();
    }

    Ok(())
}

async fn colorize_images(
    base_i: &str,
    base_o: &str,
    target_color: [u8; 3],
) -> Result<(), opencv::Error> {

    if !std::path::Path::new(&base_i).exists() {
        println!("Error: Image '{}' does not exist.", base_i);
    }


    match extract_desired_areas(&base_i) {
        Ok(mask) => {
            imgcodecs::imwrite(&base_o, &mask, &opencv::core::Vector::<i32>::new()).unwrap();
            println!("Desired areas extraction successful");
        }
        Err(err) => println!("Error: {}", err),
    }

    let mut original_image = match imgcodecs::imread(&base_i, imgcodecs::IMREAD_COLOR) {
        Ok(img) => img,
        Err(err) => {
            println!("Error reading image '{}' {}", base_i, err);
            return Err(err)
        }
    };

    let mask = match imgcodecs::imread(&base_o, imgcodecs::IMREAD_GRAYSCALE) {
        Ok(mask) => mask,
        Err(err) => {
            println!("Error reading mask {}", err);
            return Err(err)

        }
    };
    let rgb = Rgb::from(target_color);
    let hsv_target = Hsl::from(&rgb);

    let hsv_target_hue = hsv_target.hue() ;
    let hsv_target_saturation = hsv_target.saturation();
    let hsv_target_value = hsv_target.lightness();

    apply_color_change(&mut original_image, &mask, hsv_target_hue, hsv_target_saturation, hsv_target_value);

    imgcodecs::imwrite(&base_i, &original_image, &opencv::core::Vector::<i32>::new()).unwrap();
    Ok(())
}

fn adjust_color(base_pixel: &Rgba<f32>, target_color: [u8; 3]) -> Rgba<f32> {
    let r = base_pixel.0[0] as f64;
    let g = base_pixel.0[1] as f64;
    let b = base_pixel.0[2] as f64;

    let target_r = target_color[0] as f64;
    let target_g = target_color[1] as f64;
    let target_b = target_color[2] as f64;

    let target_rgb = Rgb::new(target_r, target_g, target_b, None);
    let target_hue = Hsl::from(target_rgb).hue();
    let target_rgb = Rgb::new(target_r, target_g, target_b, None);
    let target_saturation = Hsl::from(target_rgb).saturation();

    let base_rgb: Rgb = Rgb::new(r, g, b, None);
    let mut base_hsl = Hsl::from(base_rgb);
    if base_hsl.lightness() > 15. && base_hsl.lightness() < 75. {
        base_hsl.set_hue(0.);
        base_hsl.set_hue(target_hue);
        base_hsl.set_saturation(0.);
        base_hsl.set_saturation(target_saturation);
    }

    let base_rgb: Rgb = colorsys::Rgb::from(&mut base_hsl);

    Rgba([
        base_rgb.red() as f32,
        base_rgb.green() as f32,
        base_rgb.blue() as f32,
        base_pixel[3],
    ])
}


fn extract_desired_areas(image_path: &str) -> Result<Mat, opencv::Error> {
    let image = imgcodecs::imread(image_path, imgcodecs::IMREAD_COLOR)?;

    let mut hsv_image = Mat::default();
    imgproc::cvt_color(&image, &mut hsv_image, imgproc::COLOR_BGR2HSV, 0)?;

    let lower_bound = Scalar::new(10.0, 1.0, 10.0, 0.0); // Lower bound for hue, saturation, and value
    let upper_bound = Scalar::new(45.0, 255.0, 255.0, 255.0); // Upper bound for hue, saturation, and valu

    let mut desired_mask = Mat::default();
    opencv::core::in_range(&hsv_image, &lower_bound, &upper_bound, &mut desired_mask)?;

    let mut desired_mask_output = Mat::default();

    let kernel = Mat::ones(1, 1, CV_8U)?;
    imgproc::morphology_ex(
        &desired_mask,
        &mut desired_mask_output,
        imgproc::MORPH_OPEN,
        &kernel,
        opencv::core::Point::new(-1, -1),
        3,
        opencv::core::BORDER_ISOLATED,
        opencv::core::Scalar::default(),
    )?;


    Ok(desired_mask_output)
}

fn apply_color_change(original_image: &mut Mat, mask: &Mat, target_hue: f64, target_saturation: f64, target_value: f64) {
    for y in 0..original_image.rows() {
        for x in 0..original_image.cols() {
            let mask_value = mask.at_2d::<u8>(y, x).unwrap();

            if *mask_value == 255 {
                let bgr_pixel = original_image.at_2d_mut::<Vec3b>(y, x).unwrap();

                let rgba = Rgb::new(bgr_pixel[2] as f64, bgr_pixel[1] as f64, bgr_pixel[0] as f64, None);
                let mut hsla: Hsl = rgba.as_ref().into();

                let original_lightness = hsla.lightness();
                let delta_lightness = target_value - original_lightness;
                let transformed_lightness = original_lightness + non_linear_transform(delta_lightness);

                hsla.set_hue(target_hue);
                hsla.set_saturation(target_saturation);
                hsla.set_lightness(transformed_lightness);


                let rgb_arr: [u8; 3] = Rgb::from(&hsla).into();

                bgr_pixel[0] = rgb_arr[2];
                bgr_pixel[1] = rgb_arr[1];
                bgr_pixel[2] = rgb_arr[0];
            }
        }
    }
}

fn non_linear_transform(delta_lightness: f64) -> f64 {
    delta_lightness.signum() * delta_lightness.abs().powf(0.87)
}

