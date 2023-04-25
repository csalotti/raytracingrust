use std::ops::{Index, IndexMut, Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Range};
use std::fmt::{Result,Display};

use rand::Rng;

#[derive(Clone, Copy)]
pub struct Vec3 {
    e:[f64; 3]
}

pub type Point3 = Vec3;
pub type Color = Vec3;

// Vectors utilities 

impl Vec3 {
    //Constructors
    pub fn new(e0: f64, e1: f64, e2: f64) -> Vec3 {
        Vec3 {
            e: [e0, e1, e2]
        }
    }
    
    pub fn random(r: Range<f64>) -> Vec3 {
        let mut rng = rand::thread_rng();

        Vec3 {
            e: [
                rng.gen_range(r.clone()),
                rng.gen_range(r.clone()),
                rng.gen_range(r)
            ]       
        }
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        // Simplification Lambertian reflexion 
        // by generating random reflexion Vector.
        // See : https://viclw17.github.io/2018/07/20/raytracing-diffuse-materials
        loop { 
            let v = Vec3::random(-1.0..1.0);
            if v.length() < 1.0 { 
                return v
            }
        }
    }

    pub fn random_in_hemisphere(normal: Vec3) -> Vec3 {
        let in_unit_sphere =Vec3::random_in_unit_sphere();
        if in_unit_sphere.dot(&normal) > 0.0 {
            in_unit_sphere
        } else {
            (-1.0) * in_unit_sphere
        }
    }
  
    // Getters
    pub fn x(&self) -> f64 {
        self[0]
    }

    pub fn y(&self) -> f64 {
        self[1]
    }

    pub fn z(&self) -> f64 {
        self[2]
    } 

    // Utilities
    pub fn dot(&self, other : &Vec3) -> f64 {
        self[0] * other[0] + self[1] * other[1] + self[2] * other[2]
    }

    pub fn length(&self) -> f64 {
        self.dot(self).sqrt()
    }

    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3 {
            e: [
                self[1] * other[2] - self[2] * other[1],
                self[2] * other[0] - self[0] * other[2],
                self[0] * other[1] - self[1] * other[0]
            ]
        }
    }

    pub fn normalized(self) -> Vec3 {
        self / self.length()
    }

    pub fn near_zero(&self) -> bool{
        const EPS: f64 = 1.0e-8;
        self[0].abs() < EPS && self[1].abs() < EPS && self[2].abs() < EPS
    }

    pub fn refect(&self, normal: &Vec3) -> Vec3 {
       *self - 2.0 * self.dot(normal) * *normal 
    }

}

impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &f64 {
        &self.e[index]
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut f64 {
        &mut self.e[index]
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            e: [self[0] + other[0], self[1] + other[1], self[2] + other[2]]
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Vec3) {
        *self = Vec3 {
            e: [self[0] + other[0], self[1] + other[1], self[2] + other[2]]
        };
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            e: [self[0] - other[0], self[1] - other[1], self[2] - other[2]]
        }
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Vec3) {
        *self = Vec3 {
            e: [self[0] - other[0], self[1] - other[1], self[2] - other[2]]
        };
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: f64) -> Vec3 {
        Vec3 {
            e: [self[0] * other, self[1] * other, self[2] * other]
        }
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, other: f64) {
        *self = Vec3 {
            e: [self[0] * other, self[1] * other, self[2] * other]
        };
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            e: [self * other[0], self * other[1], self * other[2]]
        }
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3{
        Vec3 { 
            e: [self[0]*other[0], self[1]*other[1], self[2]*other[2]] 
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, other: f64) -> Vec3 {
        Vec3 {
            e: [self[0] / other, self[1] / other, self[2] / other]
        }
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, other: f64) {
        *self = Vec3 {
            e: [self[0] / other, self[1] / other, self[2] / other]
        };
    }
}

impl Display for Vec3 {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result {
       write!(f, "({}, {}, {})", self[0], self[1], self[2])
   } 
}

// Color utilities

impl Color {
    pub fn format_color(&self, samples_per_pixels: u64) -> String {
        let ir = (256.0 * (self[0] / (samples_per_pixels as f64)).sqrt().clamp(0.0, 0.999)) as u64;
        let ig = (256.0 * (self[1] / (samples_per_pixels as f64)).sqrt().clamp(0.0, 0.999)) as u64;
        let ib = (256.0 * (self[2] / (samples_per_pixels as f64)).sqrt().clamp(0.0, 0.999)) as u64;

        format!("{} {} {}", ir, ig, ib) 
    }
}