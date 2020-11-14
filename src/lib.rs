mod vec3;
mod ray;

fn color(ray: &ray::Ray) -> vec3::Vec3 {
    let unit_direction = vec3::XYZ::new(vec3::unit_vector(ray.direction()));
    let t = 0.5 * (unit_direction.y() + 1.0);
    return vec3::Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + vec3::Vec3::new(0.5, 0.7, 1.0) * t;
}

pub fn generate_ppm_string() -> String {
    let x_pixels = 200;
    let y_pixels = 100;
    let max_rgb_range = 255;

    // Header of ppm file
    let format = "P3";
    let mut header = String::new();
    header = header + format + "\n";
    header = header + &x_pixels.to_string() + " " + &y_pixels.to_string() + "\n";
    header = header + &max_rgb_range.to_string();

    let mut body = String::new();

    // Set up the "scene"
    let lower_left_corner = vec3::Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = vec3::Vec3::new(4.0, 0.0, 0.0);
    let vertical = vec3::Vec3::new(0.0, 2.0, 0.0);
    let origin = vec3::Vec3::zero();

    for j in (0..y_pixels).rev() {
        for i in 0..x_pixels {
            // 0 ~ 1.0
            let r = i as f64 / x_pixels as f64;
            let g = j as f64/ y_pixels as f64;
            let b = 0.2;
            let rgb = vec3::RGB::new(r, g, b);

            // 0 ~ 255
            let ir = (255.99 * rgb.r()) as i64;
            let ig = (255.99 * rgb.g()) as i64;
            let ib = (255.99 * rgb.b()) as i64;
            body = body + &ir.to_string() + " " + &ig.to_string() + " " + &ib.to_string() + "\n";
        }
    }

    return header + "\n" + &body;
}
