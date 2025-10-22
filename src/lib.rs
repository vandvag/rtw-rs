use rand::Rng;

pub mod camera;
pub mod hittable;
pub mod ray;

fn random() -> glam::DVec3 {
    let mut rng = rand::rng();
    glam::DVec3::new(rng.random(), rng.random(), rng.random())
}

fn random_range(range: std::ops::Range<f64>) -> glam::DVec3 {
    let mut rng = rand::rng();
    glam::DVec3::new(
        rng.random_range(range.clone()),
        rng.random_range(range.clone()),
        rng.random_range(range.clone()),
    )
}

fn random_unit_vector() -> glam::DVec3 {
    loop {
        let p = random_range(-1.0..1.0);
        let lensq = p.length_squared();
        if lensq > 1e-160 || lensq <= 1.0 {
            return p.normalize();
        }
    }
}

fn random_on_hemisphere(normal: glam::DVec3) -> glam::DVec3 {
    let on_unit_sphere = random_unit_vector();

    if on_unit_sphere.dot(normal) > 0.0 {
        return on_unit_sphere;
    }

    -on_unit_sphere
}
mod utils;

