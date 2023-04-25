use super::ray::Ray;
use super::hit::HitRecord;
use super::vec::{Color, Vec3};
pub trait Scatter {
    fn scatter(&self, r_in : &Ray, rec: &HitRecord) -> Option<(Color, Ray)>;
}

pub struct Lambertian {
    albedo: Color
}

impl Lambertian {
    pub fn new(a: Color) -> Self {
        Lambertian {
            albedo : a
        }
    }
}

impl Scatter for Lambertian {
    fn scatter(&self, _r_in : &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let mut scatter_direction = rec.normal + Vec3::random_in_unit_sphere().normalized();
        if scatter_direction.near_zero() {
            // catch degenerate direction
            scatter_direction = rec.normal;
        }
        
        let scattered = Ray::new(rec.p, scatter_direction);

        Some((self.albedo, scattered))
    }
}

pub struct Metal {
    albedo: Color,
    fuzz : f64
}

impl Metal{
    pub fn new(albedo: Color, fuzz : f64) -> Self {
        Metal {
            albedo,
            fuzz
        }
    }
}

impl Scatter for Metal{
    fn scatter(&self, r_in : &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let reflected = r_in.direction().refect(rec.normal).normalized(); 
        let scattered = Ray::new(rec.p, reflected + self.fuzz * Vec3::random_in_unit_sphere());
        if scattered.direction().dot(rec.normal) > 0.0 {
            Some((self.albedo, scattered)) 
        } else {
            None
        }
    }
}
