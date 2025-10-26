use std::fmt::Display;

use glam::DVec3;
use rand::Rng;

use crate::{
    hittable::Hittable,
    ray::Ray,
    utils::{gamma::linear_to_gamma, vec::random_in_unit_disk},
};
use indicatif::{ProgressIterator, ProgressStyle};
use itertools::Itertools;

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

pub struct CameraBuilder {
    aspect_ratio: f64,
    image_width: u32,
    samples_per_pixel: u32,
    max_depth: u32,
    vfov: f64,
    look_from: DVec3,
    look_at: DVec3,
    vup: DVec3,
    defocus_angle: f64,
    defocus_distance: f64,
}

impl Default for CameraBuilder {
    fn default() -> Self {
        Self {
            aspect_ratio: 16.0 / 9.0,
            image_width: 400,
            samples_per_pixel: 10,
            max_depth: 10,
            vfov: 90.0,
            look_from: DVec3::ZERO,
            look_at: DVec3::new(0.0, 0.0, -1.0),
            vup: DVec3::new(0.0, 1.0, 0.0),
            defocus_angle: 0.0,
            defocus_distance: 10.0,
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

        let center = self.look_from;

        let theta = self.vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * self.defocus_distance;
        let viewport_width = viewport_height * (self.image_width as f64 / image_height as f64);

        let w = (self.look_from - self.look_at).normalize();
        let u = DVec3::cross(self.vup, w).normalize();
        let v = DVec3::cross(w, u);

        let viewport_u = viewport_width * u;
        let viewport_v = viewport_height * -v;

        let pixel_delta_u = viewport_u / self.image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        let viewport_upper_left =
            center - (viewport_u / 2.0) - (viewport_v / 2.0) - (self.defocus_distance * w);
        let pixel00_location = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);
        let defocus_radius = self.defocus_distance * (self.defocus_angle.to_radians()).tan();
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;

        Camera {
            aspect_ratio: self.aspect_ratio,
            image_width: self.image_width,
            image_height,
            center,
            pixel00_location,
            pixel_delta_u,
            pixel_delta_v,
            samples_per_pixel: self.samples_per_pixel,
            max_depth: self.max_depth,
            vfov: self.vfov,
            look_from: self.look_from,
            look_at: self.look_at,
            vup: self.vup,
            u,
            v,
            w,
            defocus_angle: self.defocus_angle,
            focus_distance: self.defocus_distance,
            defocus_disk_u,
            defocus_disk_v,
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

    pub fn vfov(mut self, vfov: f64) -> Self {
        self.vfov = vfov;
        self
    }

    pub fn look_from(mut self, look_from: DVec3) -> Self {
        self.look_from = look_from;
        self
    }

    pub fn look_at(mut self, look_at: DVec3) -> Self {
        self.look_at = look_at;
        self
    }

    pub fn vup(mut self, vup: DVec3) -> Self {
        self.vup = vup;
        self
    }

    pub fn defocus_angle(mut self, angle: f64) -> Self {
        self.defocus_angle = angle;
        self
    }

    pub fn defocus_distance(mut self, distance: f64) -> Self {
        self.defocus_distance = distance;
        self
    }
}

fn sample_square() -> DVec3 {
    let mut rng = rand::rng();
    DVec3::new(rng.random::<f64>() - 0.5, rng.random::<f64>() - 0.5, 0.0)
}
