use glam::Vec3A;
use std::rc::Rc;

use crate::{ray::Ray, scene::Scene};

pub struct Tracer;

impl Tracer {
    pub fn trace(ray: &Ray, camera_pos: Vec3A, scene: &Scene, depth: i32) -> Vec3A {
        if let Some((h, hshape)) = scene.shapes.try_intersect(&ray) {
            let mut diffuse = Vec3A::ZERO;
            let mut specular = Vec3A::ZERO;
            for light in &scene.light_sources {
                let shadow_ray = Ray::new(
                    h.intersection_at,
                    (light.position - h.intersection_at).normalize(),
                );
                let mut exclude_light = false;
                for shape in &scene.shapes.shapes {
                    if Rc::ptr_eq(shape, &hshape) {
                        continue;
                    };
                    if shape.hits(&shadow_ray).is_some_and(|t| t > 0.0 && t < 1.0) {
                        exclude_light = true;
                    }
                }

                if exclude_light {
                    continue;
                }

                let light_vec = (light.position - h.intersection_at).normalize();
                if h.surface_normal.dot(light_vec) >= 0.0 {
                    diffuse += h.material.diffuse_constant
                        * light.diffuse_intensity
                        * h.surface_normal.dot(light_vec);
                    let reflectance =
                        2.0 * h.surface_normal.dot(light_vec) * h.surface_normal - light_vec;
                    let view = camera_pos - h.intersection_at;
                    specular += h.material.specular_constant
                        * light.specular_intensity
                        * (view.dot(reflectance)).powf(h.material.shininess_factor)
                }
            }
            let reflected = if depth > 0 {
                let view_cap = (ray.direction * (-1.0)).normalize();
                let reflectance =
                    2.0 * h.surface_normal.dot(view_cap) * h.surface_normal - view_cap;
                let reflected_ray =
                    Ray::new(h.intersection_at + h.surface_normal * (0.01), reflectance);
                Self::trace(&reflected_ray, camera_pos, scene, depth - 1) * h.material.reflectivity
            } else {
                Vec3A::ZERO
            };

            (reflected
                + h.color
                + diffuse
                + specular
                + scene.ambient_light * h.material.ambient_constant)
                .clamp(Vec3A::ZERO, Vec3A::ONE)
        } else {
            Vec3A::ZERO
        }
    }
}
