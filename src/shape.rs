use glam::Vec3A;
use std::rc::Rc;

use super::ray::Ray;

pub struct Material {
    pub ambient_constant: Vec3A,
    pub diffuse_constant: Vec3A,
    pub specular_constant: Vec3A,
    pub shininess_factor: f32,
    pub reflectivity: Vec3A,
}

impl Material {
    pub fn new(
        ambient_constant: Vec3A,
        diffuse_constant: Vec3A,
        specular_constant: Vec3A,
        shininess_factor: f32,
        reflectivity: Vec3A,
    ) -> Self {
        Self {
            ambient_constant,
            diffuse_constant,
            specular_constant,
            shininess_factor,
            reflectivity,
        }
    }
}

pub trait Shape {
    fn hits(&self, ray: &Ray) -> Option<f32>;
    fn hits_at(&self, ray: &Ray, at: f32) -> Hit;
}

pub struct Sphere {
    center: Vec3A,
    radius: f32,
    color: Vec3A,
    material: Rc<Material>,
}

pub struct Shapes {
    pub shapes: Vec<Rc<dyn Shape>>,
}

#[derive(Clone)]
pub struct Hit {
    pub at: f32,
    pub intersection_at: Vec3A,
    pub surface_normal: Vec3A,
    pub material: Rc<Material>,
    pub color: Vec3A,
}

impl Shapes {
    pub fn new() -> Self {
        Self { shapes: Vec::new() }
    }

    pub fn add(&mut self, shape: Rc<dyn Shape>) {
        self.shapes.push(shape)
    }

    pub fn try_intersect(&self, ray: &Ray) -> Option<(Hit, Rc<dyn Shape>)> {
        let mut t = Some(f32::INFINITY);
        let mut closest_hit = None;
        for shape in &self.shapes {
            let shape_hit = shape.hits(ray);
            if shape_hit.is_some() && shape_hit < t {
                t = shape_hit;
                closest_hit = Some(shape)
            }
        }
        closest_hit.map(|shape| (shape.hits_at(ray, t.unwrap()), shape.clone()))
    }
}

impl Sphere {
    pub fn new(center: Vec3A, radius: f32, color: Vec3A, material: Rc<Material>) -> Self {
        Self {
            center,
            radius,
            color,
            material,
        }
    }
}

impl Shape for Sphere {
    fn hits(&self, ray: &Ray) -> Option<f32> {
        let a = ray.direction.length_squared();
        let b = 2.0 * (ray.origin - self.center).dot(ray.direction);
        let c = (ray.origin - self.center).length_squared() - self.radius.powi(2);

        let discriminant = b.powi(2) - 4.0 * a * c;

        // info!("{discriminant} {b} {a} {c}");

        if discriminant < 0.0 {
            None
        } else {
            let t1 = (-b + f32::sqrt(discriminant)) / (2.0 * a);
            let t2 = (-b - f32::sqrt(discriminant)) / (2.0 * a);
            // info!(t1, t2);
            let result = if t1 < 0.0 {
                if t2 < 0.0 {
                    None
                } else {
                    Some(t2)
                }
            } else if t2 < 0.0 {
                if t1 < 0.0 {
                    None
                } else {
                    Some(t1)
                }
            } else {
                Some(t1.min(t2))
            };
            // info!(result);
            result
        }
    }
    fn hits_at(&self, ray: &Ray, at: f32) -> Hit {
        let t = at;
        let intersection_at = ray.origin + t * ray.direction;
        let surface_normal = (intersection_at - self.center).normalize();
        let material = self.material.clone();
        let color = self.color;

        Hit {
            at: t,
            intersection_at,
            surface_normal,
            material,
            color,
        }
    }
}
