use glam::DVec3;

use crate::camera::Camera;

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
