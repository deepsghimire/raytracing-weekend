use glam::Vec3A;

use super::ray::Ray;

use tracing::info;

pub trait Shape {
    fn intersects_at(&self, ray: &Ray) -> Option<Hit>;
}

pub struct Sphere {
    center: Vec3A,
    radius: f32,
    color: Vec3A,
}

pub struct Shapes {
    shapes: Vec<Box<dyn Shape>>,
}

#[derive(Clone, Copy)]
pub struct Hit {
    pub at: f32,
    pub color: Vec3A,
}

impl Shapes {
    pub fn new() -> Self {
        Self { shapes: Vec::new() }
    }

    pub fn add(&mut self, shape: Box<dyn Shape>) {
        self.shapes.push(shape)
    }
}

impl Shape for Shapes {
    fn intersects_at(&self, ray: &Ray) -> Option<Hit> {
        self.shapes.iter().fold(None, |a, b| {
            let t = b.intersects_at(ray);
            a.map_or(t, |v| {
                t.map_or(Some(v), |x| if x.at < v.at { Some(x) } else { Some(v) })
            })
        })
    }
}

impl Sphere {
    pub fn new(center: Vec3A, radius: f32, color: Vec3A) -> Self {
        Self {
            center,
            radius,
            color,
        }
    }
}

impl Shape for Sphere {
    fn intersects_at(&self, ray: &Ray) -> Option<Hit> {
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
            info!(t1, t2);
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
            info!(result);
            Some(Hit {
                at: result?,
                color: self.color,
            })
        }
    }
}
