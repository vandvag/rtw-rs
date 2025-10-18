use glam::DVec3;

pub struct Camera {
    /// Ratio of image width over image height
    pub aspect_ratio: f64,
    /// Rendered image width in pixel count
    pub image_width: u32,
    /// Calculated image height
    pub image_height: u32,
    /// Center of the camera
    pub center: DVec3,
    /// Location of pixel 0, 0
    pub pixel00_location: DVec3,
    /// Offset to pixel to the right
    pub pixel_delta_u: DVec3,
    /// Offset to pixel below
    pub pixel_delta_v: DVec3,
}

impl Camera {
    pub fn init() -> CameraBuilder {
        CameraBuilder::default()
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
            aspect_ratio: self.aspect_ratio,
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
