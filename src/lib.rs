/// Implements functionality for a Ray Tracer as per the book Ray Tracing in one weekend.
///

mod vec3;
mod ray;

/// Blends white and blue depending on the up/down of the Y coordinate.
fn color(ray: &ray::Ray) -> vec3::RGB {
    // Make it an unit vector so that -1 < y < 1.
    let unit_direction = vec3::XYZ::new(vec3::unit_vector(ray.direction()));

    // When t is 1.0 we have blue, then t is 0.0 we have white.
    let t = 0.5 * (unit_direction.y() + 1.0);

    return vec3::RGB::new(
        vec3::Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) +
        vec3::Vec3::new(0.5, 0.7, 1.0) * t);
}

/// Generates an image in the ppm format.
///
/// Entry point for our experiments! Read https://en.wikipedia.org/wiki/Netpbm for more information
/// on the ppm format.
pub fn generate_ppm_string() -> String {
    // Set up the "scene"
    let x_pixels = 200;
    let y_pixels = 100;
    let max_rgb_range = 255;

    let lower_left_corner = vec3::Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = vec3::Vec3::new(4.0, 0.0, 0.0);
    let vertical = vec3::Vec3::new(0.0, 2.0, 0.0);
    let origin = vec3::Vec3::zero();

    // Header of ppm file
    let format = "P3";
    let mut header = String::new();
    header = header + format + "\n";
    header = header + &x_pixels.to_string() + " " + &y_pixels.to_string() + "\n";
    header = header + &max_rgb_range.to_string();

    // Body of the ppm image.
    let mut body = String::new();

    for j in (0..y_pixels).rev() {
        for i in 0..x_pixels {
            // Calculate the direction of the ray.
            let u = i as f64 / x_pixels as f64;
            let v = j as f64/ y_pixels as f64;
            let direction = &lower_left_corner + &(&horizontal * u) + &vertical * v;

            // The ray goes from the camera origin (0, 0, 0) towards the scene.
            // the direction moves as our for loops goes through x, y.
            let ray = ray::Ray::new(&origin, &direction);
            let color = color(&ray);

            // scale back to 0 ~ 255.
            let ir = (255.99 * color.r()) as i64;
            let ig = (255.99 * color.g()) as i64;
            let ib = (255.99 * color.b()) as i64;

            // Write to the body of the image.
            body = body + &ir.to_string() + " " + &ig.to_string() + " " + &ib.to_string() + "\n";
        }
    }

    return header + "\n" + &body;
}
