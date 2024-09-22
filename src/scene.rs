use super::ray::Ray;
use super::shape::Shape;
use super::shape::Shapes;
use glam::Vec3A;

pub struct Light {
    pub position: Vec3A,
    pub diffuse_intensity: Vec3A,
    pub specular_intensity: Vec3A,
}

pub struct Scene {
    pub shapes: Shapes,
    pub light_sources: Vec<Light>,
    pub ambient_light: Vec3A,
}

impl Scene {
    pub fn new(ambient_light: Vec3A) -> Self {
        let shapes = Shapes::new();
        let light_sources = Vec::new();
        Self {
            shapes,
            light_sources,
            ambient_light,
        }
    }
}
