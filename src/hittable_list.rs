use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use std::collections::VecDeque;
use std::rc::Rc;

pub struct HittableList {
    objects: VecDeque<Rc<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: VecDeque::new(),
        }
    }

    pub fn new_init(object: Rc<dyn Hittable>) -> HittableList {
        let mut objects = VecDeque::new();
        objects.push_back(object);
        Self { objects }
    }

    pub fn clear(&mut self) {
        self.objects.clear()
    }

    pub fn add(&mut self, object: Rc<dyn Hittable>) {
        self.objects.push_back(object)
    }

    pub fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, record: &mut HitRecord) -> bool {
        let mut temp_record = HitRecord::new();
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
