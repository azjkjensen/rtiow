use crate::euclidean::{Color, Ray, Vec3};
use crate::hittable::HitRecord;
use rand::random;

#[derive(Clone, Copy)]
pub enum Material {
    NoMaterial,
    Lambertian { albedo: Color },
    Metal { albedo: Color, fuzz_in: f64 },
    Dialectric { ir: f64 },
}

impl Default for Material {
    fn default() -> Self {
        Self::NoMaterial
    }
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
                *scattered = Ray::new(record.p, scatter_direction);
                *attenuation = *albedo;
                true
            }
            Self::Metal { albedo, fuzz_in: _ } => {
                let reflected = ray_in.dir.unit_vector().reflect(&record.normal);
                let fuzz = self.fuzz().unwrap_or(0.0);
                *scattered = Ray::new(record.p, reflected + Vec3::random_in_unit_sphere() * fuzz);
                *attenuation = *albedo;
                scattered.dir.dot(&record.normal) > 0.0
            }
            Self::Dialectric { ir } => {
                *attenuation = Color::new(1.0, 1.0, 1.0);
                *scattered = {
                    let refraction_ratio = if record.front_face { 1.0 / ir } else { *ir };
                    let unit_direction = ray_in.dir.unit_vector();
                    let cos_theta = (-unit_direction).dot(&record.normal).min(1.0);
                    let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

                    let cannot_refract = refraction_ratio * sin_theta > 1.0;
                    let direction = if cannot_refract
                        || Self::reflectance(cos_theta, refraction_ratio) > random()
                    {
                        unit_direction.reflect(&record.normal)
                    } else {
                        unit_direction.refract(&record.normal, refraction_ratio)
                    };
                    Ray::new(record.p, direction)
                };
                true
            }
            Self::NoMaterial => false,
        }
    }

    #[inline(always)]
    pub fn fuzz(&self) -> Option<f64> {
        match self {
            Self::Metal { albedo: _, fuzz_in } => {
                if *fuzz_in < 1.0 {
                    Some(*fuzz_in)
                } else {
                    Some(1.0)
                }
            }
            _ => None,
        }
    }

    #[inline(always)]
    pub fn reflectance(cosine: f64, ref_index: f64) -> f64 {
        let mut r0 = (1.0 - ref_index) / (1.0 + ref_index);
        r0 *= r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
    }
}
