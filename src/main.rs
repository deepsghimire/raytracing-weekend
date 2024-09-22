#![allow(dead_code)]

use std::{error::Error, io};

use glam::Vec3A;
use ray::Ray;
use scene::Light;
use std::rc::Rc;
use tracing::{info, Level};
use tracing_subscriber;

const ASPECT_RATIO: f32 = 16f32 / 9f32;
mod img;
mod ray;
mod scene;
mod shape;
mod utils;

use scene::Scene;
use shape::Material;
use shape::Sphere;
use utils::as_color;

fn main() -> Result<(), Box<dyn Error>> {
    let collector = tracing_subscriber::fmt()
        .compact()
        .with_writer(std::io::stderr)
        .with_max_level(Level::TRACE)
        .with_ansi(false)
        .without_time()
        .finish();
    tracing::subscriber::set_global_default(collector).expect("logging error");

    let image_width = 1200;
    let image_height = (image_width as f32 / ASPECT_RATIO) as i32;
    info!(ASPECT_RATIO);
    info!(image_width);
    info!(image_height);
    let mat1 = Rc::new(Material::new(Vec3A::new(0.8, 0.8, 0.8), 1.0, 0.01, 1.0));
    let mat2 = Rc::new(Material::new(Vec3A::new(0.8, 0.8, 0.8), 0.88, 0.15, 1.0));
    let mut scene = Scene::new(Vec3A::new(0.2, 0.2, 0.3));
    scene.shapes.add(Rc::new(Sphere::new(
        Vec3A::new(-1.0, 0.0, 2.0),
        1.0,
        Vec3A::new(1.0, 0.0, 1.0),
        mat2.clone(),
    )));
    scene.shapes.add(Rc::new(Sphere::new(
        Vec3A::new(1.0, -1.0, 3.0),
        1.0,
        Vec3A::new(0.2, 0.2, 0.5),
        mat1.clone(),
    )));
    scene.shapes.add(Rc::new(Sphere::new(
        Vec3A::new(-1.0, 2.0, 10.0),
        4.5,
        Vec3A::new(0.2, 0.2, 0.5),
        mat1.clone(),
    )));
    scene.light_sources.push(Light {
        position: Vec3A::new(1.0, 1.0, 1.5),
        diffuse_intensity: Vec3A::new(1.0, 0.0, 0.0),
        specular_intensity: Vec3A::new(0.5, 0.5, 0.5),
    });

    scene.light_sources.push(Light {
        position: Vec3A::new(1.0, 0.0, -1.0),
        diffuse_intensity: Vec3A::new(0.0, 0.1, 0.0),
        specular_intensity: Vec3A::new(1.0, 1.0, 1.0),
    });
    let camera_pos = Vec3A::new(0.0, 0.0, -1.0);
    let viewport_top_left = Vec3A::new(1.0, 1.0 / ASPECT_RATIO, 0.0);
    let viewport_top_right = Vec3A::new(-1.0, 1.0 / ASPECT_RATIO, 0.0);
    let viewport_bottom_left = Vec3A::new(1.0, -1.0 / ASPECT_RATIO, 0.0);
    let viewport_bottom_right = Vec3A::new(-1.0, -1.0 / ASPECT_RATIO, 0.0);
    info!("top left: {}", viewport_top_left);
    info!("top right: {}", viewport_top_right);
    info!("bottom left: {}", viewport_bottom_left);
    info!("bottom right: {}", viewport_bottom_right);

    let mut img_encoder = img::PPMEncoder::new(image_width, image_height);

    for row in 0..image_height {
        for col in 0..image_width {
            // info!(row, col, "Working for...");
            let lerp_factor_row = row as f32 / image_height as f32;
            let lerp_factor_width = col as f32 / image_width as f32;

            let t = Vec3A::lerp(viewport_top_left, viewport_top_right, lerp_factor_width);
            let b = Vec3A::lerp(
                viewport_bottom_left,
                viewport_bottom_right,
                lerp_factor_width,
            );
            let pixel = Vec3A::lerp(t, b, lerp_factor_row);
            let ray = ray::Ray::new(pixel, (pixel - camera_pos).normalize());

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
                let result = h.color
                    + diffuse
                    + specular
                    + scene.ambient_light * h.material.ambient_constant;

                img_encoder.put_pixel_color(as_color(&result.clamp(Vec3A::ZERO, Vec3A::ONE)));
            } else {
                // let (r, g, b) = as_color(&scene.ambient_light);
                img_encoder.put_pixel_color(as_color(&scene.ambient_light))
            }
        }
    }
    img_encoder.write_to(io::BufWriter::new(io::stdout()))?;
    Ok(())
}
