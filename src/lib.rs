mod pairs;
mod vec2;

use pairs::Pairs;

pub use vec2::Vec2;

pub const RED: Color = Color(255, 0, 0);
pub const PINK: Color = Color(255, 125, 125);

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Copy, Clone)]
pub struct Color(pub u8, pub u8, pub u8);

impl From<Color> for u32 {
    fn from(value: Color) -> Self {
        let r: u32 = value.0.into();
        let g: u32 = value.1.into();
        let b: u32 = value.2.into();
        (r << 16) + (g << 8) + b
    }
}

#[wasm_bindgen]
pub struct Particle {
    pub pos: Vec2,
    pub vel: Vec2,
    pub radius: f32,
}

#[wasm_bindgen]
impl Particle {
    pub fn new(pos: Vec2, vel: Vec2, radius: f32) -> Self {
        Self { pos, vel, radius }
    }

    fn mass(&self) -> f32 {
        self.radius * self.radius
    }

    pub fn contains(&self, p: Vec2) -> bool {
        Vec2::dist(self.pos, p) < self.radius
    }

    fn step(&mut self, dt: f32, drag: f32) {
        self.pos = self.pos + self.vel * dt;
        self.vel = self.vel * drag;
        self.vel = self.vel + Vec2(0.0, 500.0) * dt;
        if self.vel.dot(self.vel).abs() < 1e-2 {
            self.vel = Vec2(0.0, 0.0);
        }
    }

    fn frame_collision(&mut self, frame: &Frame) {
        if let Some((coll_x, dx)) = frame.collide_x(self) {
            self.vel.0 = self.vel.0 * -1.0 * 0.95;

            let edge = self.pos.0 - dx * self.radius;
            self.pos.0 = self.pos.0 + (coll_x - edge);
        }

        if let Some((coll_y, dy)) = frame.collide_y(self) {
            self.vel.1 = self.vel.1 * -1.0 * 0.95;

            let edge = self.pos.1 - dy * self.radius;
            self.pos.1 = self.pos.1 + (coll_y - edge);
        }
    }

    fn collision(&self, other: &Particle) -> bool {
        let radius_sum = self.radius + other.radius;
        Vec2::dist_squared(self.pos, other.pos) < (radius_sum * radius_sum)
    }

    fn collide(&self, other: &Particle) -> Option<((Vec2, Vec2), (Vec2, Vec2))> {
        if self.collision(other) {
            Some((
                Particle::new_vel(self, other),
                Particle::new_pos(self, other),
            ))
        } else {
            None
        }
    }

    fn new_vel(p1: &Particle, p2: &Particle) -> (Vec2, Vec2) {
        // As measured for p1 (self)
        let dpos = p1.pos - p2.pos;
        let coeff = Vec2::dot(p1.vel - p2.vel, dpos) / Vec2::dot(dpos, dpos);

        let m1 = p1.mass();
        let m2 = p2.mass();
        let m_coeff1 = 2.0 * m1 / (m1 + m2);
        let m_coeff2 = 2.0 * m2 / (m1 + m2);

        let dvel = dpos * coeff;
        (p1.vel - dvel * m_coeff1, p2.vel + dvel * m_coeff2)
    }

    fn new_pos(p1: &Particle, p2: &Particle) -> (Vec2, Vec2) {
        let axis = p2.pos - p1.pos;
        let dist = axis.mag();
        let half_overlap = 0.5 * (p1.radius + p2.radius - dist);
        let axis_norm = axis / dist;
        let displacement = axis_norm * half_overlap;
        (p1.pos - displacement, p2.pos + displacement)
    }
}

struct Frame {
    top_left: Vec2,
    bottom_right: Vec2,
}

impl Frame {
    fn collide_x(&self, part: &Particle) -> Option<(f32, f32)> {
        if part.pos.0 - part.radius < self.top_left.0 {
            Some((self.top_left.0, 1.0))
        } else if part.pos.0 + part.radius > self.bottom_right.0 {
            Some((self.bottom_right.0, -1.0))
        } else {
            None
        }
    }

    fn collide_y(&self, part: &Particle) -> Option<(f32, f32)> {
        if part.pos.1 - part.radius < self.top_left.1 {
            Some((self.top_left.1, 1.0))
        } else if part.pos.1 + part.radius > self.bottom_right.1 {
            Some((self.bottom_right.1, -1.0))
        } else {
            None
        }
    }
}

#[wasm_bindgen]
pub struct World {
    frame: Frame,
    // TODO: Do these need to be public, or can we have a method?
    particles: Vec<Particle>,
    colors: Vec<Color>,
}

#[wasm_bindgen]
impl World {
    pub fn new(width: usize, height: usize) -> Self {
        let frame = Frame {
            top_left: Vec2(0.0, 0.0),
            bottom_right: Vec2(width as f32, height as f32),
        };

        Self {
            frame,
            particles: Vec::new(),
            colors: Vec::new(),
        }
    }

    pub fn momentum(&self) -> f32 {
        self.particles.iter().map(|p| p.vel.mag() * p.mass()).sum()
    }

    pub fn num_particles(&self) -> usize {
        self.particles.len()
    }

    pub fn particles(&self) -> *const Particle {
        self.particles.as_ptr()
    }

    pub fn colors(&self) -> *const Color {
        self.colors.as_ptr()
    }

    /// Adds the particle to the world if the space is unoccupied.
    pub fn try_push(&mut self, particle: Particle) -> bool {
        // TODO: We can probably remove this check and let it resolve the static collision
        for p in &self.particles {
            if p.collision(&particle) {
                return false;
            }
        }
        self.particles.push(particle);
        self.colors.push(RED);
        true
    }

    pub fn step_frame(&mut self, dt: f32, drag: f32, steps: usize) -> u32 {
        let sub_dt = dt / (steps as f32);

        let mut collision_checks = 0;
        for _ in 0..steps {
            collision_checks += self.step_dt(sub_dt, drag);
        }

        collision_checks
    }

    pub fn step_dt(&mut self, dt: f32, drag: f32) -> u32 {
        for part in self.particles.iter_mut() {
            part.step(dt, drag);
        }

        let mut collision_checks = 0;
        for (i1, i2) in Pairs::new(0..self.particles.len()) {
            // Split the array into non-overlapping slices to convince the borrow checker
            // that p1 and p2 are pointing to different particles.
            // TODO: `Pairs` should handle this and yield mut references when iterating.
            let (fst, rem) = self.particles.split_at_mut(i2);
            let p1 = &mut fst[i1];
            let p2 = &mut rem[0];

            if let Some(((vel1, vel2), (pos1, pos2))) = p1.collide(p2) {
                p1.vel = vel1 * 0.99;
                p2.vel = vel2 * 0.99;

                p1.pos = pos1;
                p2.pos = pos2;

                self.colors[i1].0 = self.colors[i1].0.wrapping_sub(1);
                self.colors[i2].2 = self.colors[i2].2.wrapping_add(1);
            }
            collision_checks += 1;
        }

        // TODO: Unify how particle-particle and particle-frame collisions are done
        for p in self.particles.iter_mut() {
            p.frame_collision(&self.frame);
        }

        collision_checks
    }
}
