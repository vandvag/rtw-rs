use glam::DVec3;
use indicatif::{ProgressIterator, ProgressStyle};
use itertools::Itertools;

fn main() {
    let image_width = 256;
    let image_height = 256;

    let progress_style = ProgressStyle::default_bar()
        .template("Raytracing! [{bar:40.cyan/blue}] {pos:>3}/{len:3}")
        .unwrap()
        .progress_chars("=>-");

    println!("P3\n{} {}\n255\n", image_width, image_height);

    let image_width_d = image_width as f64;
    let image_height_d = image_height as f64;

    let pixels = (0..image_height)
        .cartesian_product(0..image_width)
        .collect::<Vec<(u64, u64)>>()
        .into_iter()
        .progress_count(image_height * image_width)
        .with_style(progress_style)
        .map(|(h, w)| {
            let r = w as f64 / (image_width_d - 1.0);
            let g = h as f64 / (image_height_d - 1.0);
            let pixel_color = DVec3::new(r, g, 0.0);

            format!(
                "{} {} {}",
                (255.999 * pixel_color.x) as u8,
                (255.999 * pixel_color.y) as u8,
                (255.999 * pixel_color.z) as u8
            )
        })
        .collect::<Vec<String>>()
        .join("\n");

    println!("{pixels}")
}
