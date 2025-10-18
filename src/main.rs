use glam::DVec3;
use rtw::{camera::Camera, hittable::sphere::Sphere};

fn main() {
    let camera = Camera::init().build();

    let world = vec![
        Sphere::new(DVec3::new(0.0, 0.0, -1.0), 0.5),
        Sphere::new(DVec3::new(0.0, -100.5, -1.0), 100.0),
    ];

    camera.render(&world);
}
