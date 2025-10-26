mod scenes;

pub mod camera;
pub mod hittable;
pub mod material;
pub mod ray;

pub mod utils;

pub fn render_scene(file_name: &str) -> std::io::Result<()> {
    crate::scenes::test_scene(file_name)
}
