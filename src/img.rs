use std::error;
use std::io;

const MAGICNUMBER: &str = "P3";
const MAXVALUE: u8 = 255;

pub struct PPMEncoder {
    pixels: Vec<(u8, u8, u8)>,
    width: i32,
    height: i32,
}

impl PPMEncoder {
    pub fn new(width: i32, height: i32) -> Self {
        Self {
            pixels: Vec::with_capacity((width * height) as usize),
            width,
            height,
        }
    }
    pub fn put_pixel(&mut self, r: u8, g: u8, b: u8) {
        self.pixels.push((r, g, b));
    }

    pub fn write_to<T: io::Write>(&self, mut dest: T) -> Result<(), Box<dyn error::Error>> {
        writeln!(dest, "{}", MAGICNUMBER)?;
        writeln!(dest, "{} {}", self.width, self.height)?;
        writeln!(dest, "{}", MAXVALUE)?;

        for &(r, g, b) in &self.pixels {
            writeln!(dest, "{} {} {}", r, g, b)?;
        }
        Ok(())
    }
}
