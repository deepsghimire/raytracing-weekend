#![allow(dead_code)]

use std::{error::Error, io};

use glam::Vec3A;
use shape::Shape;
use tracing::{info, instrument, Level};
use tracing_subscriber;

const ASPECT_RATIO: f32 = 16f32 / 9f32;
mod img;
mod ray;
mod shape;

use shape::Shapes;
use shape::Sphere;

fn main() -> Result<(), Box<dyn Error>> {
    let collector = tracing_subscriber::fmt()
        .compact()
        .with_writer(std::io::stderr)
        .with_max_level(Level::TRACE)
        .with_ansi(false)
        .without_time()
        .finish();
    tracing::subscriber::set_global_default(collector).expect("logging error");

    let image_width = 400;
    let image_height = (image_width as f32 / ASPECT_RATIO) as i32;
    info!(ASPECT_RATIO);
    info!(image_width);
    info!(image_height);
    let mut scene = Shapes::new();
    scene.add(Box::new(Sphere::new(
        Vec3A::new(-1.0, 0.0, 4.5),
        4.0,
        Vec3A::new(0.5, 0.5, 0.5),
    )));
    scene.add(Box::new(Sphere::new(
        Vec3A::new(1.0, 0.0, 4.0),
        3.9,
        Vec3A::new(0.2, 0.2, 0.5),
    )));
    let camera_pos = Vec3A::new(0.0, 0.0, -0.1);
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

            if let Some(h) = scene.intersects_at(&ray) {
                let [r, g, b] = h.color.to_array();
                let r = (r * 255.0) as u8;
                let g = (g * 255.0) as u8;
                let b = (b * 255.0) as u8;
                img_encoder.put_pixel(r, g, b)
            } else {
                img_encoder.put_pixel(0, 0, 0)
            }
        }
    }
    img_encoder.write_to(io::BufWriter::new(io::stdout()))?;
    Ok(())
}
