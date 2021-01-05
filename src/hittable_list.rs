use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use std::collections::VecDeque;
use std::sync::Arc;

#[derive(Default)]
pub struct HittableList {
    objects: VecDeque<Arc<dyn Hittable + Sync + Send>>,
}

impl HittableList {
    pub fn new_init(object: Arc<dyn Hittable + Sync + Send>) -> HittableList {
        let mut objects = VecDeque::new();
        objects.push_back(object);
        Self { objects }
    }

    pub fn clear(&mut self) {
        self.objects.clear()
    }

    pub fn add(&mut self, object: Arc<dyn Hittable + Sync + Send>) {
        self.objects.push_back(object)
    }

    pub fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, record: &mut HitRecord) -> bool {
        let mut temp_record = HitRecord::default();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for object in &self.objects {
            if object.hit(ray, t_min, closest_so_far, &mut temp_record) {
                hit_anything = true;
                closest_so_far = temp_record.t;
                *record = temp_record;
            }
        }
        hit_anything
    }
}
