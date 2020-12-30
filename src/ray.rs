use crate::vec3::{Point3, Vec3};

pub struct Ray {
    pub origin: Point3,
    pub dir: Vec3,
}

impl Ray {
    pub fn new() -> Self {
        Ray {
            origin: Point3::new(),
            dir: Vec3::new(),
        }
    }

    pub fn new_init(origin: Point3, dir: Vec3) -> Self {
        Ray { origin, dir }
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.origin + (self.dir * t)
    }
}
