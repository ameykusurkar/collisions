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

        // coeff >= 0 means the particles are already separating; bouncing them
        // would send them back into each other.
        if coeff >= 0.0 {
            return (p1.vel, p2.vel);
        }

        let m1 = p1.mass();
        let m2 = p2.mass();
        // Each particle's velocity change is scaled by the *other* particle's mass.
        let m_coeff1 = 2.0 * m2 / (m1 + m2);
        let m_coeff2 = 2.0 * m1 / (m1 + m2);

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
            // Only bounce if moving into the segment; if the particle is already
            // moving away, reflecting would send it back in.
            let new_vel = if part.vel.dot(normal) < 0.0 {
                reflect(part.vel, normal)
            } else {
                part.vel
            };
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

    /// Clears all particles from the world.
    pub fn clear_particles(&mut self) {
        self.particles.clear();
        self.colors.clear();
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

#[cfg(test)]
mod tests {
    use super::*;

    fn momentum(p1: &Particle, v1: Vec2, p2: &Particle, v2: Vec2) -> Vec2 {
        v1 * p1.mass() + v2 * p2.mass()
    }

    fn assert_close(a: Vec2, b: Vec2) {
        assert!(
            (a.0 - b.0).abs() < 1e-2 && (a.1 - b.1).abs() < 1e-2,
            "expected ({}, {}) to be close to ({}, {})",
            a.0,
            a.1,
            b.0,
            b.1
        );
    }

    #[test]
    fn collision_conserves_momentum_for_unequal_masses() {
        let big = Particle::new(Vec2(0.0, 0.0), Vec2(100.0, 0.0), 20.0);
        let small = Particle::new(Vec2(24.0, 0.0), Vec2(0.0, 0.0), 5.0);

        let (v1, v2) = Particle::new_vel(&big, &small);

        assert_close(
            momentum(&big, big.vel, &small, small.vel),
            momentum(&big, v1, &small, v2),
        );
        // A heavy particle keeps moving forward after hitting a light one.
        assert!(v1.0 > 0.0);
        assert!(v2.0 > v1.0);
    }

    #[test]
    fn overlapping_particles_moving_apart_keep_their_velocities() {
        let p1 = Particle::new(Vec2(0.0, 0.0), Vec2(-10.0, 0.0), 10.0);
        let p2 = Particle::new(Vec2(15.0, 0.0), Vec2(10.0, 0.0), 10.0);

        let (v1, v2) = Particle::new_vel(&p1, &p2);

        assert_close(v1, p1.vel);
        assert_close(v2, p2.vel);
    }

    #[test]
    fn particle_moving_into_segment_bounces() {
        let floor = LineSegment::new(Vec2(0.0, 100.0), Vec2(200.0, 100.0));
        let part = Particle::new(Vec2(50.0, 95.0), Vec2(0.0, 50.0), 10.0);

        let (vel, pos) = floor.collide(&part).unwrap();

        assert_close(vel, Vec2(0.0, -50.0));
        assert_close(pos, Vec2(50.0, 90.0));
    }

    #[test]
    fn overlapping_particle_moving_away_from_segment_keeps_its_velocity() {
        let floor = LineSegment::new(Vec2(0.0, 100.0), Vec2(200.0, 100.0));
        let part = Particle::new(Vec2(50.0, 95.0), Vec2(0.0, -50.0), 10.0);

        let (vel, pos) = floor.collide(&part).unwrap();

        assert_close(vel, part.vel);
        assert_close(pos, Vec2(50.0, 90.0));
    }
}
