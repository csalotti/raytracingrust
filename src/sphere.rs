use std::sync::Arc;

use super::material::Scatter;

use super::vec::{Vec3, Point3};
use super::ray::Ray;
use super::hit::{Hit, HitRecord};

pub struct Sphere {
    center : Point3,
    radius : f64,
    mat: Arc<dyn Scatter>
}

impl Sphere {
    pub fn new(center : Point3, radius:f64, mat: Arc<dyn Scatter>) -> Sphere{
        Sphere {
            center,
            radius,
            mat
        }
    }
}

impl Hit for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {

        // To know if ray cross sphere, resoluve equation :
        // (𝐏−𝐂)⋅(𝐏−𝐂)=𝑟^2        
        // (𝐏(𝑡)−𝐂)⋅(𝐏(𝑡)−𝐂)=𝑟^2
        // (𝐀+𝑡𝐛−𝐂)⋅(𝐀+𝑡𝐛−𝐂)=𝑟^2
        // 𝑡^2𝐛⋅𝐛+2𝑡𝐛⋅(𝐀−𝐂)+(𝐀−𝐂)⋅(𝐀−𝐂)−𝑟^2=0 
        let oc = r.origin() - self.center;
        let a = r.direction().length().powi(2);
        let half_b = oc.dot(r.direction());
        let c = oc.length().powi(2) - self.radius.powi(2);
       
        // By quadratic resolution : 
        //  - ∆ = 0 : One solution 
        //  - ∆ > 0 : Two real solutions
        //  - ∆ < 0 : imaginaries solutions
        let discriminant = half_b.powi(2) -  a * c;
        if discriminant < 0.0 { 
            return None;
        }

        // Find nearest root that lies in the acceptable range
        let mut root = (-half_b - discriminant.sqrt())/ a;
        if root < t_min || root > t_max {
            root = (-half_b + discriminant.sqrt()) / a;
            if root < t_min || root > t_max{
                return None
            }
        }
        let p = r.at(root);
        let mut rec = HitRecord {
            t : root,
            p,
            mat: Arc::clone(&self.mat),
            normal: Vec3::new(0.0, 0.0, 0.0),
            front_face: false
        };

        // Inwards or outwards attribution by dot product result
        let outward_normal = (rec.p - self.center)/ self.radius;
        rec.set_face_normal(r, outward_normal);

        Some(rec)
    } 

}
