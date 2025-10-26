use glam::DVec3;
use rand::Rng;
use std::sync::Arc;

use rtw::{
    camera::Camera,
    hittable::sphere::Sphere,
    material::{dielectric::Dielectric, lambertian::Lambertian, metal::Metal},
    utils,
};

fn scene1() {
    let camera = Camera::init()
        .aspect_ratio(16.0 / 9.0)
        .image_width(400)
        .max_depth(10)
        .samples_per_pixel(10)
        .look_from(DVec3::new(-2.0, 2.0, 1.0))
        .look_at(DVec3::new(0.0, 0.0, -1.0))
        .vup(DVec3::new(0.0, 1.0, 0.0))
        .build();

    let material_ground = Arc::new(Lambertian {
        albedo: DVec3::new(0.8, 0.8, 0.0),
    });

    let material_center = Arc::new(Lambertian {
        albedo: DVec3::new(0.1, 0.2, 0.5),
    });

    let material_left = Arc::new(Dielectric {
        refraction_index: 1.5,
    });

    let material_bubble = Arc::new(Dielectric {
        refraction_index: 1.00 / 1.50,
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
        Sphere::new(DVec3::new(-1.0, 0.0, -1.0), 0.5, material_left.clone()),
        Sphere::new(DVec3::new(-1.0, 0.0, -1.0), 0.4, material_bubble.clone()),
        Sphere::new(DVec3::new(1.0, 0.0, -1.0), 0.5, material_right.clone()),
    ];

    camera.render(&world);
}

fn scene2() {
    let mut world: Vec<Sphere> = vec![];

    let ground_material = Lambertian {
        albedo: DVec3::new(0.5, 0.5, 0.5),
    };
    world.push(Sphere::new(
        DVec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(ground_material),
    ));

    let mut rng = rand::rng();

    for a in -11..11 {
        for b in -11..11 {
            let mat_choice: f64 = rng.random();
            let center = DVec3::new(
                a as f64 + 0.9 * rng.random::<f64>(),
                0.2,
                b as f64 + 0.9 * rng.random::<f64>(),
            );

            let random_point = DVec3::new(4.0, 0.2, 0.0);
            if (center - random_point).length() > 0.9 {
                if mat_choice < 0.8 {
                    let mat = Lambertian {
                        albedo: utils::vec::random() * utils::vec::random(),
                    };
                    world.push(Sphere {
                        center,
                        radius: 0.2,
                        mat: Arc::new(mat),
                    })
                } else if mat_choice < 0.9 {
                    let albedo = utils::vec::random_range(0.5..1.0);
                    let fuzz: f64 = rng.random_range(0.0..0.5);
                    let mat = Metal { albedo, fuzz };
                    world.push(Sphere {
                        center,
                        radius: 0.2,
                        mat: Arc::new(mat),
                    })
                } else {
                    let mat = Dielectric {
                        refraction_index: 1.5,
                    };
                    world.push(Sphere {
                        center,
                        radius: 0.2,
                        mat: Arc::new(mat),
                    })
                }
            }
        }
    }

    let mat1 = Dielectric {
        refraction_index: 1.5,
    };
    world.push(Sphere::new(DVec3::new(0.0, 1.0, 0.0), 1.0, Arc::new(mat1)));

    let mat2 = Lambertian {
        albedo: DVec3::new(0.4, 0.2, 0.1),
    };
    world.push(Sphere::new(DVec3::new(-4.0, 1.0, 0.0), 1.0, Arc::new(mat2)));

    let mat3 = Metal {
        albedo: DVec3::new(0.7, 0.6, 0.5),
        fuzz: 0.0,
    };
    world.push(Sphere::new(DVec3::new(4.0, 1.0, 0.0), 1.0, Arc::new(mat3)));

    let cam = Camera::init()
        .aspect_ratio(16.0 / 9.0)
        .image_width(1200)
        .samples_per_pixel(500)
        .max_depth(50)
        .vfov(20.0)
        .look_from(DVec3::new(13.0, 2.0, 3.0))
        .look_at(DVec3::new(0.0, 0.0, 0.0))
        .vup(DVec3::new(0.0, 1.0, 0.0))
        .defocus_angle(0.6)
        .defocus_distance(10.0)
        .build();

    cam.render(&world);
}

fn main() {
    scene1();
}
