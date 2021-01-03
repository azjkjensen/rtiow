mod camera;
mod hittable;
mod hittable_list;
mod material;
mod ray;
mod sphere;
mod util;
mod vec3;

use camera::Camera;
use hittable::HitRecord;
use hittable_list::HittableList;
use indicatif::ProgressBar;
use material::Material;
use rand::random;
use ray::Ray;
use rayon::prelude::*;
use sphere::Sphere;
use std::sync::Arc;
use vec3::{Color, Point3, Vec3};

fn ray_color(ray: &Ray, world: &HittableList, depth: u32) -> Color {
    // if we have reached maximum depth, stop collecting light
    if depth <= 0 {
        return Color::new_init(0.0, 0.0, 0.0);
    }
    let mut record = HitRecord::new();
    let res = if world.hit(&ray, 0.001, f64::INFINITY, &mut record) {
        let mut scattered = Ray::new();
        let mut attenuation = Color::new();
        return if record
            .material
            .scatter(ray, &record, &mut attenuation, &mut scattered)
        {
            ray_color(&scattered, &world, depth - 1) * attenuation
        } else {
            Color::new_init(0.0, 0.0, 0.0)
        };
    } else {
        let unit_direction = ray.dir.unit_vector();
        let t = 0.5 * (unit_direction.y() + 1.0);
        Color::new_init(1.0, 1.0, 1.0) * (1.0 - t) + Color::new_init(0.5, 0.7, 1.0) * t
    };
    res
}

fn main() {
    // image
    let aspect_ratio = 16.0 / 9.0;
    let image_width: u64 = 400;
    let image_height: u64 = (image_width as f64 / aspect_ratio) as u64;
    let samples_per_pixel = 100;
    let max_bounce_depth = 50;

    // world
    let mut world = HittableList::new();
    let material_ground = Material::Lambertian {
        albedo: Color::new_init(0.8, 0.8, 0.0),
    };
    let material_center = Material::Lambertian {
        albedo: Color::new_init(0.7, 0.3, 0.3),
    };
    let material_left = Material::Metal {
        albedo: Color::new_init(0.8, 0.8, 0.8),
    };
    let material_right = Material::Metal {
        albedo: Color::new_init(0.8, 0.6, 0.2),
    };
    world.add(Arc::new(Sphere::new_init(
        Point3::new_init(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    )));
    world.add(Arc::new(Sphere::new_init(
        Point3::new_init(0.0, 0.0, -1.0),
        0.5,
        material_center,
    )));
    world.add(Arc::new(Sphere::new_init(
        Point3::new_init(-1.0, 0.0, -1.0),
        0.5,
        material_left,
    )));
    world.add(Arc::new(Sphere::new_init(
        Point3::new_init(1.0, 0.0, -1.0),
        0.5,
        material_right,
    )));

    // camera
    let cam = Camera::new();

    // render
    let progress_bar = ProgressBar::new(image_height);
    print!("P3\n{} {}\n255\n", image_width, image_height);
    for j in (0..image_height).rev() {
        progress_bar.inc(1);
        for i in 0..image_width {
            let pixel_color: Color = (0..samples_per_pixel)
                .into_par_iter()
                .map(|_| {
                    let u = (i as f64 + random::<f64>()) / (image_width - 1) as f64;
                    let v = (j as f64 + random::<f64>()) / (image_height - 1) as f64;
                    let ray = cam.get_ray(u, v);
                    ray_color(&ray, &world, max_bounce_depth)
                })
                .reduce(|| Color::new(), |acc, c| acc + c);
            print!(
                "{}",
                pixel_color.as_multisample_color_str(samples_per_pixel)
            );
        }
    }
    progress_bar.finish();
}
