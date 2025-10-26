mod scenes;

pub mod camera;
pub mod hittable;
pub mod material;
pub mod ray;

pub mod utils;

pub struct RenderConfig {
    pub single_threaded: bool,
    pub output_file: String,
}

pub fn render_scene(config: &RenderConfig) -> std::io::Result<()> {
    crate::scenes::test_scene(config)
}
