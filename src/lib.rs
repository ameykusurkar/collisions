use itertools::Itertools;

mod vec2;

pub use vec2::Vec2;

pub const RED: Color = Color(255, 0, 0);
pub const PINK: Color = Color(255, 125, 125);

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

pub struct Particle {
    pub pos: Vec2,
    pub vel: Vec2,
    pub radius: f32,
}

impl Particle {
    fn mass(&self) -> f32 {
        self.radius * self.radius
    }

    pub fn contains(&self, p: Vec2) -> bool {
        Vec2::dist(self.pos, p) < self.radius
    }

    fn step(&mut self, dt: f32) {
        self.pos = self.pos + self.vel * dt;
    }

    fn frame_collision(&mut self, frame: &Frame) {
        if let Some(cx) = frame.collide_x(self) {
            self.vel.0 = self.vel.0 * -1.0;

            let axis = self.pos.0 - cx;
            let dist = axis.abs();
            let overlap = self.radius - dist + 1e-5;
            let axis_norm = axis / dist;
            self.pos.0 = self.pos.0 + axis_norm * overlap;
        }

        if let Some(cy) = frame.collide_y(self) {
            self.vel.1 = self.vel.1 * -1.0;

            let axis = self.pos.1 - cy;
            let dist = axis.abs();
            let overlap = self.radius - dist + 1e-5;
            let axis_norm = axis / dist;
            self.pos.1 = self.pos.1 + axis_norm * overlap;
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
    fn collide_x(&self, part: &Particle) -> Option<f32> {
        if part.pos.0 - part.radius < self.top_left.0 {
            Some(self.top_left.0)
        } else if part.pos.0 + part.radius > self.bottom_right.0 {
            Some(self.bottom_right.0)
        } else {
            None
        }
    }

    fn collide_y(&self, part: &Particle) -> Option<f32> {
        if part.pos.1 - part.radius < self.top_left.1 {
            Some(self.top_left.1)
        } else if part.pos.1 + part.radius > self.bottom_right.1 {
            Some(self.bottom_right.1)
        } else {
            None
        }
    }
}

pub struct World {
    frame: Frame,
    // TODO: Do these need to be public, or can we have a method?
    pub particles: Vec<Particle>,
    pub colors: Vec<Color>,
}

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

    /// Adds the particle to the world if the space is unoccupied.
    pub fn try_push(&mut self, particle: Particle) -> Result<(), ()> {
        // TODO: We can probably remove this check and let it resolve the static collision
        for p in &self.particles {
            if p.collision(&particle) {
                return Err(());
            }
        }
        self.particles.push(particle);
        self.colors.push(RED);
        Ok(())
    }

    pub fn step_frame(&mut self, dt: f32) {
        for part in self.particles.iter_mut() {
            part.step(dt);
        }

        let mut new_vels: Vec<Option<Vec2>> = vec![None; self.particles.len()];
        let mut new_pos: Vec<Option<Vec2>> = vec![None; self.particles.len()];

        for range_set in (0..self.particles.len()).combinations(2) {
            let i1 = range_set[0];
            let i2 = range_set[1];
            let part1 = &self.particles[i1];
            let part2 = &self.particles[i2];

            // TODO: Can a particle collide with multiple particles per frame?
            if let Some(((vel1, vel2), (pos1, pos2))) = &part1.collide(part2) {
                new_vels[i1] = Some(*vel1);
                new_vels[i2] = Some(*vel2);
                new_pos[i1] = Some(*pos1);
                new_pos[i2] = Some(*pos2);
            }
        }

        // TODO: Unify how particle-particle and particle-frame collisions are done
        for (i, part) in self.particles.iter_mut().enumerate() {
            if let Some(v) = new_vels[i] {
                part.vel = v;
                self.colors[i].0 = self.colors[i].0.wrapping_sub(10);
                self.colors[i].2 = self.colors[i].2.wrapping_add(10);
            } else {
                part.frame_collision(&self.frame);
            }

            if let Some(p) = new_pos[i] {
                part.pos = p;
            }
        }
    }
}
