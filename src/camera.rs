use std::fmt::Display;

use glam::DVec3;
use rand::Rng;

use crate::{hittable::Hittable, linear_to_gamma, ray::Ray};
use indicatif::{ProgressIterator, ProgressStyle};
use itertools::Itertools;

pub struct Camera {
    /// Ratio of image width over image height
    _aspect_ratio: f64,
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
}

struct Color(DVec3);

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let intensity = 0.000..0.999;
        let r = (256.0 * linear_to_gamma(self.0.x).clamp(intensity.start, intensity.end)) as u8;
        let g = (256.0 * linear_to_gamma(self.0.y).clamp(intensity.start, intensity.end)) as u8;
        let b = (256.0 * linear_to_gamma(self.0.z).clamp(intensity.start, intensity.end)) as u8;

        write!(f, "{} {} {}", r, g, b)
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
        let progress_style = ProgressStyle::default_bar()
            .template("Raytracing! [{bar:40.cyan/blue}] {pos:>3}/{len:3}")
            .expect("??")
            .progress_chars("=>-");

        println!("P3\n{} {}\n255\n", self.image_width, self.image_height);

        let scale_factor = (self.samples_per_pixel as f64).recip();

        let pixels = (0..self.image_height)
            .cartesian_product(0..self.image_width)
            .collect::<Vec<(u32, u32)>>()
            .into_iter()
            .progress_count(self.image_height as u64 * self.image_width as u64)
            .with_style(progress_style)
            .map(|(h, w)| {
                let pixel_color = (0..self.samples_per_pixel)
                    .map(|_| self.get_ray(w, h).color(world, self.max_depth))
                    .sum::<DVec3>()
                    * scale_factor;

                format!("{}", Color(pixel_color))
            })
            .collect::<Vec<String>>()
            .join("\n");

        println!("{pixels}")
    }

    fn get_ray(&self, width: u32, height: u32) -> Ray {
        let offset = sample_square();
        let pixel_sample = self.pixel00_location
            + ((width as f64 + offset.x) * self.pixel_delta_u)
            + ((height as f64 + offset.y) * self.pixel_delta_v);

        Ray {
            origin: self.center,
            direction: pixel_sample - self.center,
        }
    }
}

pub struct CameraBuilder {
    aspect_ratio: f64,
    image_width: u32,
    samples_per_pixel: u32,
    max_depth: u32,
}

impl Default for CameraBuilder {
    fn default() -> Self {
        Self {
            aspect_ratio: 16.0 / 9.0,
            image_width: 400,
            samples_per_pixel: 10,
            max_depth: 10,
        }
    }
}

impl CameraBuilder {
    pub fn build(&self) -> Camera {
        let image_height = if (self.image_width as f64) / self.aspect_ratio < 1.0 {
            1
        } else {
            ((self.image_width as f64) / self.aspect_ratio) as u32
        };
        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (self.image_width as f64 / image_height as f64);
        let camera_center = DVec3::ZERO;
        let viewport_u = DVec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = DVec3::new(0.0, -viewport_height, 0.0);
        let pixel_delta_u = viewport_u / self.image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;
        let viewport_upper_left = camera_center
            - DVec3::new(0.0, 0.0, focal_length)
            - viewport_u / 2.0
            - viewport_v / 2.0;
        let pixel00_location = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        Camera {
            _aspect_ratio: self.aspect_ratio,
            image_width: self.image_width,
            image_height,
            center: DVec3::ZERO,
            pixel00_location,
            pixel_delta_u,
            pixel_delta_v,
            samples_per_pixel: self.samples_per_pixel,
            max_depth: self.max_depth,
        }
    }

    pub fn image_width(mut self, image_width: u32) -> Self {
        self.image_width = image_width;
        self
    }

    pub fn aspect_ratio(mut self, aspect_ratio: f64) -> Self {
        self.aspect_ratio = aspect_ratio;
        self
    }

    pub fn samples_per_pixel(mut self, samples_per_pixel: u32) -> Self {
        self.samples_per_pixel = samples_per_pixel;
        self
    }

    pub fn max_depth(mut self, max_depth: u32) -> Self {
        self.max_depth = max_depth;
        self
    }
}

fn sample_square() -> DVec3 {
    let mut rng = rand::rng();
    DVec3::new(rng.random::<f64>() - 0.5, rng.random::<f64>() - 0.5, 0.0)
}
