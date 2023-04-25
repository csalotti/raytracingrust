use std::sync::Arc;

use super::material::Scatter;

use super::{Vec3, Point3, ray::Ray};

pub type World = Vec<Box<dyn Hit>>;

pub struct HitRecord {
    pub p : Point3,
    pub normal : Vec3,
    pub mat: Arc<dyn Scatter>,
    pub t : f64,
    pub front_face: bool,
}

pub trait Hit : Send + Sync {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>; 
}


impl Hit for World {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut tmp_rec = None;
        let mut closest_so_far = t_max;

        for object in self {
            if let Some(rec) = object.hit(r, t_min, closest_so_far) {
                closest_so_far = rec.t;
                tmp_rec = Some(rec);
            }
        }

        tmp_rec
    }
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        self.front_face =  r.direction().dot(outward_normal) < 0.0;
        self.normal = if self.front_face { outward_normal } else { -1.0 * outward_normal };
    }
}
