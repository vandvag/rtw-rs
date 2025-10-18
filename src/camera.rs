use glam::DVec3;

use crate::{hittable::Hittable, ray::Ray};
use indicatif::ProgressIterator;
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
}

impl Camera {
    pub fn init() -> CameraBuilder {
        CameraBuilder::default()
    }

    pub fn render<T>(&self, world: &T)
    where
        T: Hittable,
    {
        println!("P3\n{} {}\n255\n", self.image_width, self.image_height);

        let pixels = (0..self.image_height)
            .cartesian_product(0..self.image_width)
            .collect::<Vec<(u32, u32)>>()
            .into_iter()
            .progress_count(self.image_height as u64 * self.image_width as u64)
            // .with_style(progress_style)
            .map(|(h, w)| {
                let pixel_center = self.pixel00_location
                    + (w as f64 * self.pixel_delta_u)
                    + (h as f64 * self.pixel_delta_v);
                let ray = Ray::new(self.center, pixel_center - self.center);
                let pixel_color = ray.color(world);

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
}
pub struct CameraBuilder {
    aspect_ratio: f64,
    image_width: u32,
}

impl Default for CameraBuilder {
    fn default() -> Self {
        Self {
            aspect_ratio: 16.0 / 9.0,
            image_width: 400,
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
}
