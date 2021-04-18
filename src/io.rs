#[allow(dead_code)]
pub fn process() {
    const IMAGE_WIDTH: usize = 256;
    const IMAGE_HEIGHT: usize = 256;
    const MAX_BRIGHTNESS: usize = 255;

    println!("P3\n{} {}\n{}", IMAGE_WIDTH, IMAGE_HEIGHT, MAX_BRIGHTNESS);
    for h in (0..IMAGE_HEIGHT).rev() {
        for w in 0..IMAGE_WIDTH {
            let r = w as f64 / (IMAGE_WIDTH - 1) as f64;
            let g = h as f64 / (IMAGE_HEIGHT - 1) as f64;
            let b = 0.25;

            let r = (255.999 * r) as i32;
            let g = (255.999 * g) as i32;
            let b = (255.999 * b) as i32;

            println!("{} {} {}", r, g, b);
        }
    }
}

#[allow(dead_code)]
pub fn process2() {}
