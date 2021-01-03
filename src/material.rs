use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::{Color, Vec3};

#[derive(Clone, Copy)]
pub enum Material {
    NoMaterial,
    Lambertian { albedo: Color },
    Metal { albedo: Color },
}

impl Material {
    pub fn scatter(
        &self,
        ray_in: &Ray,
        record: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        match self {
            Self::Lambertian { albedo } => {
                let mut scatter_direction = record.normal + Vec3::random_unit_vector();
                if scatter_direction.near_zero() {
                    // degenerate scatter direction
                    scatter_direction = record.normal;
                }
                *scattered = Ray::new_init(record.p, scatter_direction);
                *attenuation = *albedo;
                true
            }
            Self::Metal { albedo } => {
                let reflected = ray_in.dir.unit_vector().reflect(&record.normal);
                *scattered = Ray::new_init(record.p, reflected);
                *attenuation = *albedo;
                scattered.dir.dot(&record.normal) > 0.0
            }
            NoMaterial => false,
        }
    }
}
