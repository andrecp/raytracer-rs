mod vec3;

pub fn generate_ppm_string() -> String {
    let x_pixels = 200;
    let y_pixels = 100;
    let format = "P3";
    let max_rgb_range = 255;

    let mut header = String::new();

    header = header + format + "\n";
    header = header + &x_pixels.to_string() + " " + &y_pixels.to_string() + "\n";
    header = header + &max_rgb_range.to_string();

    let mut body = String::new();

    for j in (0..y_pixels).rev() {
        for i in 0..x_pixels {
            // 0 ~ 1.0
            let r = i as f64 / x_pixels as f64;
            let g = j as f64/ y_pixels as f64;
            let b = 0.2;

            // 0 ~ 255
            let ir = (255.99 * r) as i64;
            let ig = (255.99 * g) as i64;
            let ib = (255.99 * b) as i64;
            body = body + &ir.to_string() + " " + &ig.to_string() + " " + &ib.to_string() + "\n";
        }
    }

    return header + "\n" + &body;
}






pub struct XYZ {}


pub struct RGB {}
