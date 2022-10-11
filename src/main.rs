use std::{thread, time};

const WIDTH: i64 = 160;
const HEIGHT: i64 = 44;

struct Cube {
    cube_width: i64,
    horizontal_offset: i64,
    distance_from_cam: f64,
    k1: i64,
    increment_speed: f64,
    a: f64,
    b: f64,
    c: f64,
    z_buffer: [f64; (WIDTH * HEIGHT) as usize],
    buffer: [char; (WIDTH * HEIGHT) as usize],
}

impl Cube {
    pub fn new(cube_width: i64, offset: i64) -> Self {
        Cube {
            cube_width,
            horizontal_offset: offset * cube_width,
            distance_from_cam: 100.0,
            k1: 40,
            increment_speed: 0.6,
            a: 0.0,
            b: 0.0,
            c: 0.0,
            z_buffer: [0.0; (WIDTH * HEIGHT) as usize],
            buffer: [' '; (WIDTH * HEIGHT) as usize],
        }
    }

    pub fn display(&mut self) {
        self.z_buffer = [0.0; (WIDTH * HEIGHT) as usize];
        self.buffer = [' '; (WIDTH * HEIGHT) as usize];

        self.update();

        print!("\x1b[H");

        let mut k = 0;
        while k < WIDTH * HEIGHT {
            if k % WIDTH != 0 {
                print!("{}", self.buffer[k as usize]);
            } else {
                print!("{}", 10 as char);
            }
            k += 1;
        }

        self.a += 0.05;
        self.b += 0.05;
        self.b += 0.01;

        let ten_millis = time::Duration::from_millis(30);
        thread::sleep(ten_millis);
    }

    fn calculate_x(&self, i: f64, j: f64, k: f64) -> f64 {
        j * self.a.sin() * self.b.sin() * self.c.cos()
            - k * self.a.cos() * self.b.sin() * self.c.cos()
            + j * self.a.cos() * self.c.sin()
            + k * self.a.sin() * self.c.sin()
            + i * self.b.cos() * self.c.cos()
    }

    fn calculate_y(&self, i: f64, j: f64, k: f64) -> f64 {
        j * self.a.cos() * self.c.cos() + k * self.a.sin() * self.c.cos()
            - j * self.a.sin() * self.b.sin() * self.c.sin()
            + k * self.a.cos() * self.b.sin() * self.c.sin()
            - i * self.b.cos() * self.c.sin()
    }

    fn calculate_z(&self, i: f64, j: f64, k: f64) -> f64 {
        k * self.a.cos() * self.b.cos() - j * self.a.sin() * self.b.cos() + i * self.b.sin()
    }

    fn calculate_for_surface(&mut self, cube_x: f64, cube_y: f64, cube_z: f64, ch: char) {
        let x = self.calculate_x(cube_x, cube_y, cube_z);
        let y = self.calculate_y(cube_x, cube_y, cube_z);
        let z = self.calculate_z(cube_x, cube_y, cube_z) + self.distance_from_cam;

        let ooz = 1.0 / z;

        let xp =
            WIDTH as f64 / 2.0 + self.horizontal_offset as f64 + self.k1 as f64 * ooz * x * 2.0;
        let yp = HEIGHT as f64 / 2.0 + self.k1 as f64 * ooz * y;

        let idx = xp as i64 + yp as i64 * WIDTH;

        if idx >= 0 && idx < WIDTH * HEIGHT {
            if ooz > self.z_buffer[idx as usize] {
                self.z_buffer[idx as usize] = ooz;
                self.buffer[idx as usize] = ch;
            }
        }
    }

    fn update(&mut self) {
        let mut cube_x = -self.cube_width as f64;

        while cube_x < self.cube_width as f64 {
            let mut cube_y = -self.cube_width as f64;

            while cube_y < self.cube_width as f64 {
                self.calculate_for_surface(
                    cube_x as f64,
                    cube_y as f64,
                    (-self.cube_width) as f64,
                    '@',
                );
                self.calculate_for_surface(
                    self.cube_width as f64,
                    cube_y as f64,
                    cube_x as f64,
                    '$',
                );
                self.calculate_for_surface(
                    (-self.cube_width) as f64,
                    cube_y as f64,
                    (-cube_x) as f64,
                    '~',
                );
                self.calculate_for_surface(
                    (-cube_x) as f64,
                    cube_y as f64,
                    self.cube_width as f64,
                    '#',
                );
                self.calculate_for_surface(
                    cube_x as f64,
                    (-self.cube_width) as f64,
                    (-cube_y) as f64,
                    ';',
                );
                self.calculate_for_surface(
                    cube_x as f64,
                    self.cube_width as f64,
                    cube_y as f64,
                    '+',
                );

                cube_y += self.increment_speed;
            }
            cube_x += self.increment_speed;
        }
    }
}

fn main() {
    print!("\x1b[2J");
    let mut cube = Cube::new(20, -2);
    loop {
        cube.display();
    }
}
