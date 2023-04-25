mod vec;
mod ray;
mod hit;
mod sphere;
mod camera;
mod material;

use rand::Rng;
use std::{io::{stderr, Write}, rc::Rc};
use vec::{Color, Point3, Vec3};
use ray::Ray;
use hit::{Hit, World};
use sphere::Sphere;
use camera::Camera;

use material::{Lambertian, Metal};


fn ray_color(r : &Ray, world : &World, depth: u64) -> Color {
    if depth == 0 {
        // if we exceed the ray bounce limit, no more light is generated
        return Color::new(0.0, 0.0, 0.0);
    }

    if let Some(rec) = world.hit(r, 0.0001, f64::INFINITY) {
        if let Some((attenuation, scattered )) = rec.mat.scatter(r, &rec) {
            attenuation * ray_color(&scattered, world, depth -1)
        } else {
            Color::new(0.0, 0.0, 0.0)
        }
    } else {
        // Blue/white gradient background
        let unit_direction = r.direction().normalized();
        let t = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - t ) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
    }
}

fn main(){
    //Image 
    const ASPECT_RATIO : f64 = 16.0/9.0;
    const IMAGE_WIDTH: u64 = 512;
    const IMAGE_HEIGHT: u64 = (512_f64 / ASPECT_RATIO) as u64;
    const SAMPLES_PER_PIXELS : u64 = 100;
    const MAX_DEPTH: u64 = 5; 
    // World
    // Materials
    let mat_ground = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let mat_center = Rc::new(Lambertian::new(Color::new(0.7, 0.3, 0.3)));
    let mat_left = Rc::new(Metal::new(Color::new(0.8, 0.8, 0.8), 0.3));
    let mat_right = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 1.0)); 
    
    // Spheres
    let sphere_ground = Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, mat_ground);
    let sphere_center = Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5, mat_center);
    let sphere_left = Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5, mat_left);
    let sphere_right = Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.5, mat_right);
    
    let world : Vec<Box<dyn Hit>> = vec![
        Box::new(sphere_ground),
        Box::new(sphere_center),
        Box::new(sphere_left),
        Box::new(sphere_right)
    ];

    // Camera
    let camera = Camera::new();

    // Image generation
    println!("P3");
    println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("255");

    let mut rng = rand::thread_rng();

    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {:3}", IMAGE_HEIGHT - j - 1);
        stderr().flush().unwrap();
        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..SAMPLES_PER_PIXELS {
                let random_u : f64 = rng.gen();
                let random_v : f64 = rng.gen();

                let u = ((i as f64) + random_u) / ((IMAGE_WIDTH - 1) as f64);
                let v = ((j as f64) + random_v) / ((IMAGE_HEIGHT - 1) as f64);

                let ray = camera.get_ray(u, v); 
                pixel_color += ray_color(&ray, &world, MAX_DEPTH);

            }
            println!("{}", pixel_color.format_color(SAMPLES_PER_PIXELS));
        }
    }
    eprintln!(" Done")
    }
