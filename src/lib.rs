/// Implements functionality for a Ray Tracer as per the book Ray Tracing in one weekend.
///

mod vec3;
mod ray;
mod sphere;
mod hitable;
mod hitable_list;

use hitable::Hitable;


/// Calculates the color in RGB of where the ray hits.
fn color(ray: &ray::Ray, hitable_world: &hitable_list::HitableList) -> vec3::RGB {

    let mut hit_record = hitable::HitRecord::new();
    if hitable_world.hit(&ray, 0.0, std::f64::MAX, &mut hit_record) {
        let x_normal = hit_record.normal[0] + 1.0;
        let y_normal = hit_record.normal[1] + 1.0;
        let z_normal = hit_record.normal[2] + 1.0;
        let vec3d = vec3::Vec3::new(x_normal, y_normal, z_normal) * 0.5;
        return vec3::RGB::new(vec3d);
    }
    // No hits, background color which is a blend of blue and white.

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
pub fn generate_ppm_image() -> String {
    // Set up the "scene"
    let x_pixels = 400;
    let y_pixels = 200;
    let max_rgb_range = 255;

    // Where the camera is pointed to.
    let lower_left_corner = vec3::Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = vec3::Vec3::new(4.0, 0.0, 0.0);
    let vertical = vec3::Vec3::new(0.0, 2.0, 0.0);

    // Where the camera is.
    let origin = vec3::Vec3::zero();

    // Add objects to the "scene"
    let sphere1 = sphere::Sphere::new(
        vec3::XYZ::new(vec3::Vec3::new(0.0, 0.0, -1.0)),
        0.5);

    let sphere2 = sphere::Sphere::new(
        vec3::XYZ::new(vec3::Vec3::new(0.0, -100.5, -1.0)),
        100.0);

    let mut world = hitable_list::HitableList::new();
    world.add(Box::new(sphere1));
    world.add(Box::new(sphere2));

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
            let direction = (&lower_left_corner + &(&horizontal * u)) + &vertical * v;

            // The ray goes from the camera origin (0, 0, 0) towards the scene.
            // the direction moves as our for loops goes through x, y.
            let ray = ray::Ray::new(&origin, &direction);
            let color = color(&ray, &world);

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
