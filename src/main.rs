use minifb::{Key, Window, WindowOptions};
use rand::distributions::{Distribution, Uniform};

mod vec2;

use vec2::Vec2;

const WIDTH: usize = 500;
const HEIGHT: usize = 500;
const FPS: usize = 60;
const FRAME_DT: f32 = 1.0 / (FPS as f32);

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    let (mut part1, mut part2) = build_scene(WIDTH, HEIGHT);

    let frame = Frame {
        top_left: Vec2(0.0, 0.0),
        bottom_right: Vec2(WIDTH as f32, HEIGHT as f32),
    };

    // Limit to max ~60 fps update rate
    window.set_target_fps(FPS);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        part1.step(FRAME_DT);
        part2.step(FRAME_DT);

        if Particle::collision(&part1, &part2) {
            let v1 = part1.new_vel(&part2);
            let v2 = part2.new_vel(&part1);
            part1.vel = v1;
            part2.vel = v2;
        } else {
            part1.frame_collision(&frame);
            part2.frame_collision(&frame);
        }

        for (x, pixel) in buffer.iter_mut().enumerate() {
            let p = Vec2((x % WIDTH) as f32, (x / WIDTH) as f32);

            let col = if part1.contains(p) {
                Color(
                    (p.0 / (WIDTH as f32) * 255.0) as u8,
                    (p.1 / (HEIGHT as f32) * 255.0) as u8,
                    0,
                )
            } else if part2.contains(p) {
                Color(
                    0,
                    (p.0 / (WIDTH as f32) * 255.99) as u8,
                    255 - (p.1 / (HEIGHT as f32) * 255.99) as u8,
                )
            } else {
                Color(255, 225, 255)
            };

            *pixel = col.into();
        }

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}

fn build_scene(width: usize, height: usize) -> (Particle, Particle) {
    let mut rng = rand::thread_rng();

    let r1 = Uniform::from(10..100).sample(&mut rng);
    let r2 = 100 - r1;

    loop {
        let part1 = Particle {
            pos: Vec2(
                Uniform::from(r1..width - r1).sample(&mut rng) as f32,
                Uniform::from(r1..height - r1).sample(&mut rng) as f32,
            ),
            radius: r1 as f32,
            vel: Vec2(
                Uniform::from(-250..250).sample(&mut rng) as f32,
                Uniform::from(-250..250).sample(&mut rng) as f32,
            ),
        };

        let part2 = Particle {
            pos: Vec2(
                Uniform::from(r2..width - r2).sample(&mut rng) as f32,
                Uniform::from(r2..height - r2).sample(&mut rng) as f32,
            ),
            radius: r2 as f32,
            vel: Vec2(250.0, 250.0) - part1.vel,
        };

        if !Particle::collision(&part1, &part2) {
            return (part1, part2);
        }
    }
}

#[derive(Copy, Clone)]
struct Color(u8, u8, u8);

impl From<Color> for u32 {
    fn from(value: Color) -> Self {
        let r: u32 = value.0.into();
        let g: u32 = value.1.into();
        let b: u32 = value.2.into();
        (r << 16) + (g << 8) + b
    }
}

struct Particle {
    pos: Vec2,
    vel: Vec2,
    radius: f32,
}

impl Particle {
    fn mass(&self) -> f32 {
        self.radius * self.radius
    }

    fn contains(&self, p: Vec2) -> bool {
        Vec2::dist(self.pos, p) < self.radius
    }

    fn step(&mut self, dt: f32) {
        self.pos = self.pos + self.vel * dt;
    }

    fn frame_collision(&mut self, frame: &Frame) {
        if collide_x(frame, self) {
            self.vel.0 = self.vel.0 * -1.0;
        }

        if collide_y(frame, self) {
            self.vel.1 = self.vel.1 * -1.0;
        }
    }

    fn collision(&self, other: &Particle) -> bool {
        Vec2::dist(self.pos, other.pos) < self.radius + other.radius
    }

    fn new_vel(&self, other: &Particle) -> Vec2 {
        let dpos = self.pos - other.pos;
        let vdot = Vec2::dot(self.vel - other.vel, dpos);
        let xdot = Vec2::dot(dpos, dpos);
        self.vel - dpos * (vdot / xdot * 2.0 * other.mass() / (self.mass() + other.mass()))
    }
}

struct Frame {
    top_left: Vec2,
    bottom_right: Vec2,
}

fn collide_x(frame: &Frame, part: &Particle) -> bool {
    part.pos.0 - part.radius < frame.top_left.0 || part.pos.0 + part.radius > frame.bottom_right.0
}

fn collide_y(frame: &Frame, part: &Particle) -> bool {
    part.pos.1 - part.radius < frame.top_left.1 || part.pos.1 + part.radius > frame.bottom_right.1
}
