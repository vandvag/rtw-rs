
use glam::DVec3;
use rand::Rng;

pub(crate) fn reflect(v: DVec3, n: DVec3) -> DVec3 {
    v - 2.0 * v.dot(n) * n
}

pub(crate) fn random() -> glam::DVec3 {
    let mut rng = rand::rng();
    glam::DVec3::new(rng.random(), rng.random(), rng.random())
}

pub(crate) fn random_range(range: std::ops::Range<f64>) -> glam::DVec3 {
    let mut rng = rand::rng();
    glam::DVec3::new(
        rng.random_range(range.clone()),
        rng.random_range(range.clone()),
        rng.random_range(range.clone()),
    )
}

pub(crate) fn random_unit_vector() -> glam::DVec3 {
    loop {
        let p = random_range(-1.0..1.0);
        let lensq = p.length_squared();
        if lensq > 1e-160 || lensq <= 1.0 {
            return p.normalize();
        }
    }
}

pub(crate) fn random_on_hemisphere(normal: glam::DVec3) -> glam::DVec3 {
    let on_unit_sphere = random_unit_vector();

    if on_unit_sphere.dot(normal) > 0.0 {
        return on_unit_sphere;
    }

    -on_unit_sphere
}

pub(crate) fn refract(uv: DVec3, n: DVec3, etai_over_etat: f64) -> DVec3 {
    let cos_theta = f64::min(n.dot(-uv), 1.0);
    let r_out_perp = etai_over_etat * (uv + cos_theta * n);
    let r_out_paralel = -((1.0 - r_out_perp.length_squared()).abs()).sqrt() * n;

    r_out_perp + r_out_paralel
}
