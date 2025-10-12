use indicatif::{ProgressBar, ProgressStyle};

pub fn run() {
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
            let b = 0.0;

            let ir = (255.999 * r) as u8;
            let ig = (255.999 * g) as u8;
            let ib = (255.999 * b) as u8;

            println!("{} {} {}", ir, ig, ib);
        }

        pb.inc(1);
    }
}
