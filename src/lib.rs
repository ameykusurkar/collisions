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

    // TODO: Just return the new velocity
    fn new_vel(p1: &Particle, p2: &Particle) -> (Vec2, Vec2) {
        // As measured for p1 (self)
        let dpos = p1.pos - p2.pos;
        let coeff = Vec2::dot(p1.vel - p2.vel, dpos) / Vec2::dot(dpos, dpos);

        let m1 = p1.mass();
        let m2 = p2.mass();
        let m_coeff1 = 2.0 * m1 / (m1 + m2);
        let m_coeff2 = 2.0 * m2 / (m1 + m2);

        let dvel = dpos * coeff;
        (-dvel * m_coeff1, dvel * m_coeff2)
    }

    // TODO: Just return the new pos
    fn new_pos(p1: &Particle, p2: &Particle) -> (Vec2, Vec2) {
        let axis = p2.pos - p1.pos;
        let dist = axis.mag();
        let half_overlap = 0.5 * (p1.radius + p2.radius - dist);
        let axis_norm = axis / dist;
        let displacement = axis_norm * half_overlap;
        (-displacement, displacement)
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

    pub fn step_frame(&mut self, dt: f32, drag: f32) {
        let num_steps = 4;
        let sub_dt = dt / (num_steps as f32);

        for _ in 0..num_steps {
            self.step_dt(sub_dt, drag);
        }
    }

    pub fn step_dt(&mut self, dt: f32, drag: f32) {
        for part in self.particles.iter_mut() {
            part.step(dt, drag);
        }

        // TODO: Update the actual particles
        let mut dvel: Vec<Vec2> = self.particles.iter().map(|p| p.vel).collect();
        let mut dpos: Vec<Vec2> = self.particles.iter().map(|p| p.pos).collect();

        for (i1, i2) in Pairs::new(self.particles.len()) {
            let part1 = &self.particles[i1];
            let part2 = &self.particles[i2];

            let part1 = Particle {
                pos: dpos[i1],
                vel: dvel[i1],
                radius: part1.radius,
            };
            let part2 = Particle {
                pos: dpos[i2],
                vel: dvel[i2],
                radius: part2.radius,
            };

            if let Some(((vel1, vel2), (pos1, pos2))) = &part1.collide(&part2) {
                dvel[i1] = dvel[i1] + *vel1;
                dvel[i2] = dvel[i2] + *vel2;
                dpos[i1] = dpos[i1] + *pos1;
                dpos[i2] = dpos[i2] + *pos2;

                self.colors[i1].0 = self.colors[i1].0.wrapping_sub(1);
                self.colors[i2].2 = self.colors[i2].2.wrapping_add(1);
                dvel[i1] = dvel[i1] * 0.99;
                dvel[i2] = dvel[i2] * 0.99;
            }
        }

        // TODO: Unify how particle-particle and particle-frame collisions are done
        for (i, part) in self.particles.iter_mut().enumerate() {
            part.vel = dvel[i];
            part.pos = dpos[i];
            part.frame_collision(&self.frame);
        }
    }
}
