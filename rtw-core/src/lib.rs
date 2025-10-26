mod scenes;

mod camera;
mod hittable;
mod material;
mod ray;
mod utils;

pub struct RenderConfig {
    pub single_threaded: bool,
    pub output_file: String,
}

pub fn render_scene(config: &RenderConfig) -> std::io::Result<()> {
    crate::scenes::random_scene(config)
}
