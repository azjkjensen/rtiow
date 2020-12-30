mod ray;
mod vec3;
use indicatif::ProgressBar;
use ray::Ray;
use vec3::{Color, Point3, Vec3};

fn ray_color(ray: &Ray) -> Color {
    let t = hit_sphere(&Point3::new_init(0.0, 0.0, -1.0), 0.5, ray);
    if t > 0.0 {
        let normal = (ray.at(t) - Vec3::new_init(0.0, 0.0, -1.0)).unit_vector();
        Color::new_init(normal.x() + 1.0, normal.y() + 1.0, normal.z() + 1.0) * 0.5
    } else {
        let unit_direction = ray.dir.unit_vector();
        let t = 0.5 * (unit_direction.y() + 1.0);
        Color::new_init(1.0, 1.0, 1.0) * (1.0 - t) + Color::new_init(0.5, 0.7, 1.0) * t
    }
}

fn hit_sphere(center: &Point3, radius: f64, ray: &Ray) -> f64 {
    let oc = ray.origin - *center;
    let a = ray.dir.dot(&ray.dir);
    let b = oc.dot(&ray.dir) * 2.0;
    let c = oc.dot(&oc) - radius.powf(2.0);
    let discriminant = b * b - a * c * 4.0;
    if discriminant < 0.0 {
        -1.0
    } else {
        (-b - discriminant.sqrt()) / (a * 2.0)
    }
}

fn main() {
    // image
    let aspect_ratio = 16.0 / 9.0;
    let image_width: u64 = 400;
    let image_height: u64 = (image_width as f64 / aspect_ratio) as u64;

    // camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new();
    let horizontal = Vec3::new_init(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new_init(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new_init(0.0, 0.0, focal_length);

    // render
    let progress_bar = ProgressBar::new(image_height);
    let mut out = format!("P3\n{} {}\n255\n", image_width, image_height);
    for j in (0..image_height).rev() {
        progress_bar.inc(1);
        for i in 0..image_width {
            let u = i as f64 / (image_width - 1) as f64;
            let v = j as f64 / (image_height - 1) as f64;
            let r = Ray::new_init(
                origin,
                lower_left_corner + horizontal * u + vertical * v - origin,
            );
            let pixel_color = ray_color(&r);

            out = format!("{}{}", out, pixel_color.as_color_str());
        }
    }
    progress_bar.finish();
    print!("{}", out);
}
