/// Implements functionality for a Ray Tracer as per the book Ray Tracing in one weekend.
///
use rand::prelude::*;

mod vec3;
mod ray;
mod sphere;
mod hitable;
mod hitable_list;
mod camera;

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
    let x_pixels = 200;
    let y_pixels = 100;
    let max_rgb_range = 255;
    let number_of_samples = 100;

    // Where the camera is pointed to.
    let camera = camera::Camera::new();

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

    let mut random_generator = rand::thread_rng();

    for j in (0..y_pixels).rev() {
        for i in 0..x_pixels {

            // For anti-alising we get 100 samples for each pixel and avg
            // It blends the background and the foreground.
            let mut col = vec3::Vec3::zero();
            for _ in 0..number_of_samples {
                // Calculate the direction of the ray.
                let u = (i as f64 + random_generator.gen_range(0.0, 1.0)) / x_pixels as f64;
                let v = (j as f64 + random_generator.gen_range(0.0, 1.0)) / y_pixels as f64;

                // The ray goes from the camera origin (0, 0, 0) towards the scene.
                // the direction moves as our for loops goes through x, y.
                let ray = camera.get_ray(u, v);
                col += color(&ray, &world).vec3().clone();
            }

            let color = col / number_of_samples as f64;

            // scale back to 0 ~ 255.
            let ir = (255.99 * color[0]) as i64;
            let ig = (255.99 * color[1]) as i64;
            let ib = (255.99 * color[2]) as i64;

            // Write to the body of the image.
            body = body + &ir.to_string() + " " + &ig.to_string() + " " + &ib.to_string() + "\n";
        }
    }

    return header + "\n" + &body;
}
