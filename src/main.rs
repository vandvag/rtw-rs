use glam::DVec3;
use indicatif::{ProgressIterator, ProgressStyle};
use itertools::Itertools;
use rtw::{camera::Camera, hittable::sphere::Sphere, ray::Ray};

fn main() {
    let progress_style = ProgressStyle::default_bar()
        .template("Raytracing! [{bar:40.cyan/blue}] {pos:>3}/{len:3}")
        .unwrap()
        .progress_chars("=>-");

    let camera = Camera::init().build();

    println!("P3\n{} {}\n255\n", camera.image_width, camera.image_height);

    let world = vec![
        Sphere::new(DVec3::new(0.0, 0.0, -1.0), 0.5),
        Sphere::new(DVec3::new(0.0, -100.5, -1.0), 100.0),
    ];

    let pixels = (0..camera.image_height)
        .cartesian_product(0..camera.image_width)
        .collect::<Vec<(u32, u32)>>()
        .into_iter()
        .progress_count(camera.image_height as u64 * camera.image_width as u64)
        .with_style(progress_style)
        .map(|(h, w)| {
            let pixel_center =
                camera.pixel00_location + (w as f64 * camera.pixel_delta_u) + (h as f64 * camera.pixel_delta_v);
            let ray = Ray::new(camera.center, pixel_center - camera.center);
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
