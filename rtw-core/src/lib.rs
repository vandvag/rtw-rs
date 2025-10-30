use thiserror::Error;

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

#[derive(Debug, Error)]
pub enum RtwError {
    #[error("Invalid interval! Inverval shouldn't have the same start and end")]
    InvalidInterval,
    #[error("Invalid sphere radius({0})! Radius must be positive")]
    InvalidRadius(f64),
    #[error("Scene {0} doesn't exist")]
    SceneNotFound(String),
    #[error("Io Error: {0}")]
    IoError(String),
}

pub type Result<T> = std::result::Result<T, RtwError>;

pub fn render_scene(scene: &str, config: &RenderConfig) -> Result<()> {
    if scene == "random" {
        crate::scenes::random_scene(config)
    } else if scene == "test" {
        crate::scenes::test_scene(config)
    } else {
        Err(RtwError::SceneNotFound(scene.to_owned()))
    }
}
