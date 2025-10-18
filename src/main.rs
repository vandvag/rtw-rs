use glam::DVec3;
use indicatif::{ProgressIterator, ProgressStyle};
use itertools::Itertools;
use rtw::{hittable::sphere::Sphere, ray::Ray};

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = if (image_width as f64) / aspect_ratio < 1.0 {
        1
    } else {
        ((image_width as f64) / aspect_ratio) as u64
    };
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
    let camera_center = DVec3::ZERO;
    let viewport_u = DVec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = DVec3::new(0.0, -viewport_height, 0.0);
    let pixel_delta_u = viewport_u / image_width as f64;
    let pixel_delta_v = viewport_v / image_height as f64;
    let viewport_upper_left =
        camera_center - DVec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel00_location = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    let progress_style = ProgressStyle::default_bar()
        .template("Raytracing! [{bar:40.cyan/blue}] {pos:>3}/{len:3}")
        .unwrap()
        .progress_chars("=>-");

    println!("P3\n{} {}\n255\n", image_width, image_height);

    let world = vec![
        Sphere::new(DVec3::new(0.0, 0.0, -1.0), 0.5),
        Sphere::new(DVec3::new(0.0, -100.5, -1.0), 100.0),
    ];

    let pixels = (0..image_height)
        .cartesian_product(0..image_width)
        .collect::<Vec<(u64, u64)>>()
        .into_iter()
        .progress_count(image_height * image_width)
        .with_style(progress_style)
        .map(|(h, w)| {
            let pixel_center =
                pixel00_location + (w as f64 * pixel_delta_u) + (h as f64 * pixel_delta_v);
            let ray = Ray::new(camera_center, pixel_center - camera_center);
            let pixel_color = ray.color(&world);

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
