mod scenes;

mod camera;
mod hittable;
mod material;
mod ray;
mod utils;

pub struct RenderConfig {
    pub multi_threaded: bool,
    pub output_file: String,
}

pub fn render_scene(scene: &str, config: &RenderConfig) -> std::io::Result<()> {
    if scene == "random" {
        crate::scenes::random_scene(config)
    } else if scene == "test" {
        crate::scenes::test_scene(config)
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "No scene like that",
        ))
    }
}
