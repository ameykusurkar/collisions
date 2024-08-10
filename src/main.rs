use minifb::{Key, Window, WindowOptions};
use rand::distributions::{Distribution, Uniform};

use collisions::{Color, Vec2, Particle, World, PINK};

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

    let mut world = World::new(WIDTH, HEIGHT);
    let mut phantom_particle: Option<Particle> = None;

    let mut particles_to_add = 10;
    while particles_to_add > 0 {
        let p = random_particle(WIDTH, HEIGHT);
        if world.try_push(p).is_ok() {
            particles_to_add -= 1;
        }
    }

    let mut running = true;
    let mut mouse_down = window.get_mouse_down(minifb::MouseButton::Left);

    // Limit to max ~60 fps update rate
    window.set_target_fps(FPS);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        if window.is_key_released(Key::Space) {
            running ^= true;
        }

        let old_mouse_down = mouse_down;
        mouse_down = window.get_mouse_down(minifb::MouseButton::Left);

        // Mouse pressed
        if !old_mouse_down && mouse_down {
            if let Some((mx, my)) = window.get_mouse_pos(minifb::MouseMode::Discard) {
                let particle = Particle {
                    pos: Vec2(mx, my),
                    vel: Vec2(0.0, 0.0),
                    radius: 15.0,
                };
                phantom_particle = Some(particle);
            }
        }

        // Mouse release
        if old_mouse_down && !mouse_down {
            if let Some(particle) = phantom_particle {
                let pos = particle.pos;
                let vel = if let Some((mx, my)) = window.get_mouse_pos(minifb::MouseMode::Clamp) {
                    (pos - Vec2(mx, my)) * 2.0
                } else {
                    Vec2(0.0, 0.0)
                };
                world
                    .try_push(Particle { vel, ..particle })
                    .unwrap_or_else(|_| println!("({}, {}) is occupied!", pos.0, pos.1));
                phantom_particle = None;
            }
        }

        if running {
            world.step_frame(FRAME_DT);
        }

        render(&mut buffer, &world, &phantom_particle);

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}

fn render(buffer: &mut Vec<u32>, world: &World, phantom_particle: &Option<Particle>) {
    for (x, pixel) in buffer.iter_mut().enumerate() {
        let p = Vec2((x % WIDTH) as f32, (x / WIDTH) as f32);

        let mut col = Color(255, 225, 255);
        for (part, part_clr) in world.particles.iter().zip(&world.colors) {
            if part.contains(p) {
                col = *part_clr;
                break;
            }
        }
        if let Some(part) = phantom_particle {
            if part.contains(p) {
                col = PINK;
            }
        }

        *pixel = col.into();
    }
}

fn random_particle(width: usize, height: usize) -> Particle {
    let mut rng = rand::thread_rng();
    let radius = 15;

    Particle {
        pos: Vec2(
            Uniform::from(radius..width - radius).sample(&mut rng) as f32,
            Uniform::from(radius..height - radius).sample(&mut rng) as f32,
        ),
        radius: radius as f32,
        vel: Vec2(350.0, 350.0),
    }
}
