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
use util::random_in_range;
use vec3::{Color, Point3, Vec3};

fn ray_color(ray: &Ray, world: &HittableList, depth: u32) -> Color {
    // if we have reached maximum depth, stop collecting light
    if depth == 0 {
        return Color::new(0.0, 0.0, 0.0);
    }
    let mut record = HitRecord::default();
    if world.hit(&ray, 0.001, f64::INFINITY, &mut record) {
        let mut scattered = Ray::default();
        let mut attenuation = Color::default();
        if record
            .material
            .scatter(ray, &record, &mut attenuation, &mut scattered)
        {
            ray_color(&scattered, &world, depth - 1) * attenuation
        } else {
            Color::new(0.0, 0.0, 0.0)
        }
    } else {
        let unit_direction = ray.dir.unit_vector();
        let t = 0.5 * (unit_direction.y() + 1.0);
        Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
    }
}

fn random_scene() -> HittableList {
    let mut world = HittableList::default();

    let ground_material = Material::Lambertian {
        albedo: Color::new(0.5, 0.5, 0.5),
    };
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    for a in (0..22).map(|v| v - 11) {
        for b in (0..22).map(|v| v - 11) {
            let material_choice: f64 = random();
            let center = Point3::new(
                a as f64 + 0.9 * random::<f64>(),
                0.2,
                b as f64 + 0.9 * random::<f64>(),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).len() > 0.9 {
                let sphere_material = if material_choice < 0.8 {
                    let albedo = Color::random() * Color::random();
                    Material::Lambertian { albedo }
                } else if material_choice < 0.95 {
                    let albedo = Color::random_in_range(0.5, 1.0);
                    let fuzz = random_in_range(0.0, 0.5);
                    Material::Metal {
                        albedo,
                        fuzz_in: fuzz,
                    }
                } else {
                    Material::Dialectric { ir: 1.5 }
                };
                world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
            }
        }
    }

    let material1 = Material::Dialectric { ir: 1.5 };
    let material2 = Material::Lambertian {
        albedo: Color::new(0.4, 0.2, 0.1),
    };
    let material3 = Material::Metal {
        albedo: Color::new(0.7, 0.6, 0.5),
        fuzz_in: 0.0,
    };
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));
    world
}

fn main() {
    // image
    let aspect_ratio = 16.0 / 9.0;
    let image_width: u64 = 200;
    let image_height: u64 = (image_width as f64 / aspect_ratio) as u64;
    let samples_per_pixel = 50;
    let max_bounce_depth = 50;

    // world
    let mut world = random_scene();

    // camera
    let look_from = Point3::new(13.0, 2.0, 3.0);
    let look_at = Point3::new(0.0, 0.0, 0.0);
    let view_up = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;
    let cam = Camera::new(
        &look_from,
        &look_at,
        &view_up,
        20.0,
        aspect_ratio,
        aperture,
        dist_to_focus,
    );

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
                .reduce(Color::default, |acc, c| acc + c);
            print!(
                "{}",
                pixel_color.as_multisample_color_str(samples_per_pixel)
            );
        }
    }
    progress_bar.finish();
}
