use glam::Vec3A;

pub fn as_color(v: &Vec3A) -> (u8, u8, u8) {
    let [r, g, b] = v.to_array();
    let r = (r * 255.0) as u8;
    let g = (g * 255.0) as u8;
    let b = (b * 255.0) as u8;
    (r, g, b)
}

pub fn random_vector(min: f32, max: f32) -> Vec3A {
    rand::random::<Vec3A>() * max + min
}
