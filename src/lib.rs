pub mod vec;
pub mod ray;

const TOL: f64 = 1e-10;

fn approx_eq(a: f64, b: f64, eps: f64) -> bool {
    (a - b).abs() < eps
}
