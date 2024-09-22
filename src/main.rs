#![allow(dead_code)]

use std::{error::Error, io};

use glam::Vec3A;
use ray::Ray;
use scene::Light;
use std::rc::Rc;
use tracer::Tracer;
use tracing::{info, Level};
use tracing_subscriber;
use utils::random_vector;

const ASPECT_RATIO: f32 = 16f32 / 9f32;
const RECURSION_DEPTH: i32 = 3;
const SAMPLING: i32 = 80;
mod img;
mod ray;
mod scene;
mod shape;
mod tracer;
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

    let image_width = 2400;
    let image_height = (image_width as f32 / ASPECT_RATIO) as i32;
    info!(ASPECT_RATIO);
    info!(image_width);
    info!(image_height);
    let mat1 = Rc::new(Material::new(
        Vec3A::new(0.8, 0.8, 0.8),
        Vec3A::new(0.5, 1.0, 1.0),
        Vec3A::new(0.1, 0.0, 0.0),
        1.0,
        Vec3A::new(0.5, 0.5, 0.5),
    ));
    let mat2 = Rc::new(Material::new(
        Vec3A::new(0.8, 0.8, 0.8),
        Vec3A::new(0.88, 0.88, 0.88),
        Vec3A::new(0.0, 0.0, 0.0),
        1.0,
        Vec3A::new(0.22, 0.22, 0.22),
    ));
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

    scene.shapes.add(Rc::new(Sphere::new(
        Vec3A::new(0.5, 0.5, 1.5),
        0.5,
        Vec3A::new(0.0, 0.5, 0.5),
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
            let ray_origin = pixel;
            let ray_direction = (pixel - camera_pos).normalize();
            let mut ray = ray::Ray::new(ray_origin, ray_direction);
            let mut result_color = Vec3A::ZERO;
            for _ in 0..SAMPLING {
                ray = ray::Ray::new(ray_origin + random_vector(0.0, 0.02), ray_direction);
                result_color += Tracer::trace(&ray, camera_pos, &scene, RECURSION_DEPTH)
            }
            result_color /= SAMPLING as f32;
            img_encoder.put_pixel_color(as_color(&Tracer::trace(
                &ray,
                camera_pos,
                &scene,
                RECURSION_DEPTH,
            )));
        }
    }
    img_encoder.write_to(io::BufWriter::new(io::stdout()))?;
    Ok(())
}
