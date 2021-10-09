use crate::wasm4::*;
use num_complex::Complex64;

const SCREEN_BUFFER_SIZE: usize = SCREEN_SIZE as usize * SCREEN_SIZE as usize;

pub struct State {
    zoom: f64,
    position: (f64, f64),
    screen: [u8; SCREEN_BUFFER_SIZE],
    frame: u64,
}

impl State {
    pub fn new() -> Self {
        State {
            zoom: 1.0,
            position: (0.0, 0.0),
            screen: [0; SCREEN_BUFFER_SIZE],
            frame: 0,
        }
    }

    pub fn update(&mut self) {
        if self.frame == 0 {
            for x in 0..SCREEN_SIZE {
                for y in 0..SCREEN_SIZE {
                    let real = (x as f64 / SCREEN_SIZE as f64) * 4.0 - 2.0;
                    let imaginary = (y as f64 / SCREEN_SIZE as f64) * 4.0 - 2.0;

                    let mut z = Complex64::new(0.0, 0.0);
                    let c = Complex64::new(real, imaginary);

                    let mut iterations = 100;
                    for i in 0..100 {
                        z = z * z + c;
                        if z.norm() > 2.0 {
                            iterations = i;
                            break;
                        }
                    }

                    self.screen[x as usize + y as usize * SCREEN_SIZE as usize] = (iterations + 3) % 4;
                }
            }
        }

        self.frame += 1;
    }

    pub fn draw(&mut self) {
        for x in 0..SCREEN_SIZE {
            for y in 0..SCREEN_SIZE {
                let color = self.screen[x as usize + y as usize * SCREEN_SIZE as usize];
                set_pixel(x as u8, y as u8, color);
            }
        }
    }
}

fn set_pixel(x: u8, y: u8, color: u8) {
    // There are 4 pixels per byte (2 bits per pixel)
    let index = (y as usize * SCREEN_SIZE as usize + x as usize) >> 2;

    // Calculate the bits iwthin the byte that coressponds to our position
    let shift = (x as u8 & 0b11) << 1;
    let mask = 0b11 << shift;

    unsafe {
        let framebuffer = FRAMEBUFFER.as_mut().expect("Could not get framebuffer reference");
        framebuffer[index] = (color << shift) | (framebuffer[index] & !mask);
    }
}
