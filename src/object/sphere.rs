use crate::euclidean::{Point3, Ray};
use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;

#[derive(Default)]
pub struct Sphere {
    center: Point3,
    radius: f64,
    material: Material,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, material: Material) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, record: &mut HitRecord) -> bool {
        let oc = ray.origin - self.center;
        let a = ray.dir.len_squared();
        let half_b = oc.dot(&ray.dir);
        let c = oc.len_squared() - self.radius.powf(2.0);

        let discriminant = half_b.powf(2.0) - a * c;
        if discriminant < 0.0 {
            false
        } else {
            let discriminant_sqrt = discriminant.sqrt();
            let mut root = (-half_b - discriminant_sqrt) / a;
            if root < t_min || root > t_max {
                root = (-half_b + discriminant_sqrt) / a;
                if root < t_min || root > t_max {
                    return false;
                }
            }
            record.t = root;
            record.p = ray.at(record.t);
            let outward_normal = (record.p - self.center) / self.radius;
            record.set_face_normal(&ray, &outward_normal);
            record.material = self.material;
            true
        }
    }
}
