use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new() -> Self {
        let aspect_ratio = 16.0 / 9.0;
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;

        let origin = Point3::new();
        let horizontal = Vec3::new_init(viewport_width, 0.0, 0.0);
        let vertical = Vec3::new_init(0.0, viewport_height, 0.0);
        let lower_left_corner =
            origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new_init(0.0, 0.0, focal_length);
        Self {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }

    #[inline(always)]
    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new_init(
            self.origin,
            self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin,
        )
    }
}
