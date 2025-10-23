use std::sync::Arc;

use glam::DVec3;
use rtw::{
    camera::Camera,
    hittable::sphere::Sphere,
    material::{dielectric::Dielectric, lambertian::Lambertian, metal::Metal},
};

fn scene1() {
    let camera = Camera::init().image_width(1000).max_depth(50).build();

    let material_ground = Arc::new(Lambertian {
        albedo: DVec3::new(0.8, 0.8, 0.0),
    });

    let material_center = Arc::new(Lambertian {
        albedo: DVec3::new(0.1, 0.2, 0.5),
    });

    let material_left = Arc::new(Dielectric {
        refraction_index: 1.0 / 1.33,
    });

    let material_right = Arc::new(Metal {
        albedo: DVec3::new(0.8, 0.6, 0.2),
        fuzz: 1.0,
    });

    let world = vec![
        Sphere::new(
            DVec3::new(0.0, -100.5, -1.0),
            100.0,
            material_ground.clone(),
        ),
        Sphere::new(DVec3::new(0.0, 0.0, -1.2), 0.5, material_center.clone()),
        Sphere::new(DVec3::new(-1.0, 0.0, -1.0), 0.5, material_right.clone()),
        Sphere::new(DVec3::new(1.0, 0.0, -1.0), 0.5, material_left.clone()),
    ];

    camera.render(&world);
}

fn main() {
    scene1();
}
