mod camera;
mod hittable;
mod hittable_list;
mod ray;
mod sphere;
mod util;
mod vec3;

use camera::Camera;
use hittable::HitRecord;
use hittable_list::HittableList;
use indicatif::ProgressBar;
use rand::random;
use ray::Ray;
use sphere::Sphere;
use std::rc::Rc;
use vec3::{Color, Point3, Vec3};

fn ray_color(ray: &Ray, world: &HittableList) -> Color {
    let mut record = HitRecord::new();
    if world.hit(&ray, 0.0, f64::INFINITY, &mut record) {
        (record.normal + Color::new_init(1.0, 1.0, 1.0)) * 0.5
    } else {
        let unit_direction = ray.dir.unit_vector();
        let t = 0.5 * (unit_direction.y() + 1.0);
        Color::new_init(1.0, 1.0, 1.0) * (1.0 - t) + Color::new_init(0.5, 0.7, 1.0) * t
    }
}

fn main() {
    // image
    let aspect_ratio = 16.0 / 9.0;
    let image_width: u64 = 400;
    let image_height: u64 = (image_width as f64 / aspect_ratio) as u64;
    let samples_per_pixel = 100;

    // world
    let mut world = HittableList::new();
    world.add(Rc::new(Sphere::new_init(
        Point3::new_init(0.0, 0.0, -1.0),
        0.5,
    )));
    world.add(Rc::new(Sphere::new_init(
        Point3::new_init(0.0, -100.5, -1.0),
        100.0,
    )));

    // camera
    let cam = Camera::new();

    // render
    let progress_bar = ProgressBar::new(image_height);
    let mut out = format!("P3\n{} {}\n255\n", image_width, image_height);
    for j in (0..image_height).rev() {
        progress_bar.inc(1);
        for i in 0..image_width {
            let mut pixel_color = Color::new();
            for _ in 0..samples_per_pixel {
                let u = (i as f64 + random::<f64>()) / (image_width - 1) as f64;
                let v = (j as f64 + random::<f64>()) / (image_height - 1) as f64;
                let ray = cam.get_ray(u, v);
                pixel_color += ray_color(&ray, &world);
                //eprintln!("adding {:?}", pixel_color);
            }
            out = format!(
                "{}{}",
                out,
                pixel_color.as_multisample_color_str(samples_per_pixel)
            );

            //let u = i as f64 / (image_width - 1) as f64;
            //let v = j as f64 / (image_height - 1) as f64;
            //let r = Ray::new_init(
            //    origin,
            //    lower_left_corner + horizontal * u + vertical * v - origin,
            //);
            //let pixel_color = ray_color(&r, &world);

            //out = format!("{}{}", out, pixel_color.as_color_str());
        }
    }
    progress_bar.finish();
    print!("{}", out);
}
