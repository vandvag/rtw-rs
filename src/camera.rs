mod builder;

use glam::DVec3;
use indicatif::{ProgressIterator, ProgressStyle};
use itertools::Itertools;
use rand::Rng;
use std::fmt::Display;

use crate::{
    camera::builder::CameraBuilder,
    hittable::Hittable,
    ray::Ray,
    utils::{gamma::linear_to_gamma, vec::random_in_unit_disk},
};

#[allow(dead_code)]
pub struct Camera {
    /// Ratio of image width over image height
    aspect_ratio: f64,
    /// Rendered image width in pixel count
    image_width: u32,
    /// Calculated image height
    image_height: u32,
    /// Center of the camera
    center: DVec3,
    /// Location of pixel 0, 0
    pixel00_location: DVec3,
    /// Offset to pixel to the right
    pixel_delta_u: DVec3,
    /// Offset to pixel below
    pixel_delta_v: DVec3,
    /// Count of random sample for each color pixel
    samples_per_pixel: u32,
    /// Maximum number of ray bounces into scene
    max_depth: u32,
    /// Vertical view angle (field of view)
    vfov: f64,
    /// Point camera is looking from
    look_from: DVec3,
    /// Point camera is looking at
    look_at: DVec3,
    /// Camera "up" direction
    vup: DVec3,
    /// X axis basis vector
    u: DVec3,
    /// Y axis basis vector
    v: DVec3,
    /// Z axis basis vector
    w: DVec3,
    /// Variation angle of rays through each pixel
    defocus_angle: f64,
    /// Distance from camera lookfrom point to plane of perfect focus
    focus_distance: f64,
    /// Defocus disk horizontal radius
    defocus_disk_u: DVec3,
    /// Defocus disk vertical radius
    defocus_disk_v: DVec3,
}

struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl From<DVec3> for Color {
    fn from(value: DVec3) -> Self {
        let intensity = 0.000..0.999;
        Self {
            r: (256.0 * linear_to_gamma(value.x).clamp(intensity.start, intensity.end)) as u8,
            g: (256.0 * linear_to_gamma(value.y).clamp(intensity.start, intensity.end)) as u8,
            b: (256.0 * linear_to_gamma(value.z).clamp(intensity.start, intensity.end)) as u8,
        }
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.r, self.g, self.b)
    }
}

impl Camera {
    pub fn init() -> CameraBuilder {
        CameraBuilder::default()
    }

    pub fn render<T>(&self, world: &T)
    where
        T: Hittable,
    {
        let pixels = self.get_pixel_string(world);

        println!(
            "P3\n{} {}\n255\n{}",
            self.image_width, self.image_height, pixels
        )
    }

    fn get_pixel_string<T>(&self, world: &T) -> String
    where
        T: Hittable,
    {
        let progress_style = ProgressStyle::default_bar()
            .template("Raytracing! [{bar:40.cyan/blue}] {pos:>3}/{len:3}")
            .expect("??")
            .progress_chars("=>-");

        (0..self.image_height)
            .cartesian_product(0..self.image_width)
            .collect::<Vec<(u32, u32)>>()
            .into_iter()
            .progress_count(self.image_height as u64 * self.image_width as u64)
            .with_style(progress_style)
            .map(|(h, w)| format!("{}", Color::from(self.render_pixel(w, h, world))))
            .collect::<Vec<String>>()
            .join("\n")
    }

    fn render_pixel<T>(&self, width: u32, height: u32, world: &T) -> DVec3
    where
        T: Hittable,
    {
        (0..self.samples_per_pixel)
            .map(|_| self.get_ray(width, height).color(world, self.max_depth))
            .sum::<DVec3>()
            / (self.samples_per_pixel as f64)
    }

    fn get_ray(&self, width: u32, height: u32) -> Ray {
        let offset = sample_square();
        let pixel_sample = self.pixel00_location
            + ((width as f64 + offset.x) * self.pixel_delta_u)
            + ((height as f64 + offset.y) * self.pixel_delta_v);

        let ray_origin = if self.defocus_angle <= 0.0 {
            self.center
        } else {
            self.defocus_disk_sample()
        };

        Ray {
            origin: ray_origin,
            direction: pixel_sample - self.center,
        }
    }

    fn defocus_disk_sample(&self) -> DVec3 {
        let p = random_in_unit_disk();
        self.center + (p.x * self.defocus_disk_u) + (p.y * self.defocus_disk_v)
    }
}

fn sample_square() -> DVec3 {
    let mut rng = rand::rng();
    DVec3::new(rng.random::<f64>() - 0.5, rng.random::<f64>() - 0.5, 0.0)
}
