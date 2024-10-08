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
#[repr(u8)]
#[derive(Copy, Clone)]
pub enum CollisionAlgorithm {
    Pairwise = 0,
    SweepAndPrune = 1,
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

    // TODO: Take in one segment rather than Vec
    fn collide_segment(&mut self, segment: &LineSegment) {
        if let Some((new_vel, new_pos)) = segment.collide(self) {
            self.vel = new_vel * 0.95;
            self.pos = new_pos;
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

fn axis_aligned_frame(top_left: Vec2, bottom_right: Vec2) -> Vec<LineSegment> {
    let width = bottom_right.0 - top_left.0;
    let height = bottom_right.1 - top_left.1;
    vec![
        // Top
        LineSegment::new(top_left, top_left + Vec2(width, 0.0)),
        // Left
        LineSegment::new(top_left, top_left + Vec2(0.0, height)),
        // Bottom
        LineSegment::new(bottom_right, bottom_right - Vec2(width, 0.0)),
        // Right
        LineSegment::new(bottom_right, bottom_right - Vec2(0.0, height)),
    ]
}

struct LineSegment {
    start: Vec2,
    n: Vec2,
}

impl LineSegment {
    fn new(start: Vec2, end: Vec2) -> Self {
        let n = end - start;
        Self { n, start }
    }

    fn closest_point(&self, p: Vec2) -> Option<Vec2> {
        let pa = p - self.start;
        let t = Vec2::dot(pa, self.n) / Vec2::dot(self.n, self.n);

        if 0.0 <= t && t <= 1.0 {
            Some(self.start + self.n * t)
        } else {
            None
        }
    }

    fn collide(&self, part: &Particle) -> Option<(Vec2, Vec2)> {
        let closest = self.closest_point(part.pos)?;

        let dist = Vec2::dist(closest, part.pos);
        if dist < part.radius {
            let normal = (part.pos - closest) / dist;
            let new_vel = reflect(part.vel, normal);
            let new_pos = closest + normal * part.radius;

            Some((new_vel, new_pos))
        } else {
            None
        }
    }
}

fn reflect(incident: Vec2, normal: Vec2) -> Vec2 {
    incident - normal * 2.0 * incident.dot(normal)
}

#[wasm_bindgen]
pub struct World {
    segments: Vec<LineSegment>,
    particles: Vec<Particle>,
    colors: Vec<Color>,
}

#[wasm_bindgen]
impl World {
    pub fn new(width: usize, height: usize) -> Self {
        let segments = axis_aligned_frame(Vec2(0.0, 0.0), Vec2(width as f32, height as f32));

        Self {
            segments,
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

    pub fn push_segment(&mut self, start: Vec2, end: Vec2) {
        self.segments.push(LineSegment::new(start, end));
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

    pub fn step_frame(&mut self, dt: f32, drag: f32, steps: usize, alg: CollisionAlgorithm) -> u32 {
        let sub_dt = dt / (steps as f32);

        let mut collision_checks = 0;
        for _ in 0..steps {
            collision_checks += self.step_dt(sub_dt, drag, alg);
        }

        collision_checks
    }

    fn step_dt(&mut self, dt: f32, drag: f32, alg: CollisionAlgorithm) -> u32 {
        for part in self.particles.iter_mut() {
            part.step(dt, drag);
        }

        let collision_checks = match alg {
            CollisionAlgorithm::Pairwise => self.collisions_pairwise(),
            CollisionAlgorithm::SweepAndPrune => self.collisions_sweep_and_prune(),
        };

        // TODO: Unify how particle-particle and particle-frame collisions are done
        for p in self.particles.iter_mut() {
            for segment in &self.segments {
                p.collide_segment(segment);
            }
        }

        collision_checks
    }

    fn collisions_pairwise(&mut self) -> u32 {
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
            }
            collision_checks += 1;
        }

        collision_checks
    }

    fn collisions_sweep_and_prune(&mut self) -> u32 {
        let mut collision_checks = 0;

        // TODO: Insertion sort is faster since most particles barely change between iterations.
        // Swaps can be done in the loop itself.
        self.particles
            .sort_unstable_by_key(|p| (p.pos.0 - p.radius) as i32);

        for i1 in 0..self.particles.len() {
            for i2 in (i1 + 1)..self.particles.len() {
                // Split the array into non-overlapping slices to convince the borrow checker
                // that p1 and p2 are pointing to different particles.
                // TODO: `Pairs` should handle this and yield mut references when iterating.
                let (fst, rem) = self.particles.split_at_mut(i2);
                let p1 = &mut fst[i1];
                let p2 = &mut rem[0];

                if p1.pos.0 + p1.radius < p2.pos.0 - p2.radius {
                    break;
                }

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
        }

        collision_checks
    }
}
