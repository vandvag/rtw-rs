use indicatif::{ProgressBar, ProgressStyle};
use rtw::vec;

fn main() {
    let image_width = 256;
    let image_height = 256;

    let pb = ProgressBar::new(image_height);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{msg} [{bar:40.cyan/blue}] {pos:>3}/{len:3}")
            .unwrap()
            .progress_chars("=>-"),
    );

    pb.set_message("Raytracing!");

    println!("P3\n{} {}\n255\n", image_width, image_height);

    let image_width_d = image_width as f64;
    let image_height_d = image_height as f64;
    for height in (0..image_height).map(|x| x as f64) {
        for width in (0..image_width).map(|x| x as f64) {
            let r = width / (image_width_d - 1.0);
            let g = height / (image_height_d - 1.0);
            let pixel_color = vec::Vec3::new(r, g, 0.0);

            println!("{}", vec::Color(pixel_color));
        }

        pb.inc(1);
    }
}
