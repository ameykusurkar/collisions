use itertools::Itertools;
use minifb::{Key, Window, WindowOptions};
use rand::distributions::{Distribution, Uniform};

mod vec2;

use vec2::Vec2;

const WIDTH: usize = 1200;
const HEIGHT: usize = 800;
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

    let mut particles = build_scene(WIDTH, HEIGHT, 15);

    let frame = Frame {
        top_left: Vec2(0.0, 0.0),
        bottom_right: Vec2(WIDTH as f32, HEIGHT as f32),
    };

    let mut colors: Vec<_> = particles.iter().map(|_| Color(0, 0, 0)).collect();

    // Limit to max ~60 fps update rate
    window.set_target_fps(FPS);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        for part in particles.iter_mut() {
            part.step(FRAME_DT);
        }

        let mut new_vels: Vec<Option<Vec2>> = vec![None; particles.len()];

        for range_set in (0..particles.len()).combinations(2) {
            let i1 = range_set[0];
            let i2 = range_set[1];
            let part1 = &particles[i1];
            let part2 = &particles[i2];

            // TODO: Can a particle collide with multiple particles per frame?
            if let Some((v1, v2)) = &part1.collide(&part2) {
                new_vels[i1] = Some(*v1);
                new_vels[i2] = Some(*v2);
            }
        }

        for (i, part) in particles.iter_mut().enumerate() {
            if let Some(v) = new_vels[i] {
                part.vel = v;
                colors[i] = Color::random();
            } else {
                part.frame_collision(&frame);
            }
        }

        for (x, pixel) in buffer.iter_mut().enumerate() {
            let p = Vec2((x % WIDTH) as f32, (x / WIDTH) as f32);

            let mut col = Color(255, 225, 255);
            for (i, part) in particles.iter().enumerate() {
                if part.contains(p) {
                    col = colors[i];
                    break;
                }
            }

            *pixel = col.into();
        }

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}

fn build_scene(width: usize, height: usize, count: u32) -> Vec<Particle> {
    loop {
        let particles: Vec<_> = (0..count).map(|_| random_particle(width, height)).collect();
        if (0..particles.len()).combinations(2).any(|range_set| {
            let i1 = range_set[0];
            let i2 = range_set[1];
            let part1 = &particles[i1];
            let part2 = &particles[i2];

            part1.collision(part2)
        }) {
            println!("Collision detected in initial state, trying again!");
            continue;
        }
        return particles;
    }
}

fn random_particle(width: usize, height: usize) -> Particle {
    let mut rng = rand::thread_rng();
    let radius = Uniform::from(20..50).sample(&mut rng);

    Particle {
        pos: Vec2(
            Uniform::from(radius..width - radius).sample(&mut rng) as f32,
            Uniform::from(radius..height - radius).sample(&mut rng) as f32,
        ),
        radius: radius as f32,
        vel: Vec2(
            Uniform::from(-250..250).sample(&mut rng) as f32,
            Uniform::from(-250..250).sample(&mut rng) as f32,
        ),
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

impl Color {
    fn random() -> Color {
        let mut rng = rand::thread_rng();
        Self(
            Uniform::from(0..255).sample(&mut rng),
            Uniform::from(0..255).sample(&mut rng),
            Uniform::from(0..255).sample(&mut rng),
        )
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

    fn collide(&self, other: &Particle) -> Option<(Vec2, Vec2)> {
        if self.collision(other) {
            Some((self.new_vel(other), other.new_vel(self)))
        } else {
            None
        }
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
