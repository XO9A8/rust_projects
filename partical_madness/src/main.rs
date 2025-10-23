use graphics::math::{add, mul_scalar, Vec2d};
use piston_window::*;
use rand::prelude::*;
use std::alloc::{GlobalAlloc, Layout, System};

#[global_allocator]
static ALLOCATOR: ReportingAllocator = ReportingAllocator;

struct ReportingAllocator;

extern "C" {
    fn printf(_: *const core::ffi::c_char, ...) -> core::ffi::c_int;
}

fn log(bytes_requested: usize, time_taken: u128) {
    let fmt_ptr = b"%u, %u\n\0".as_ptr().cast();

    unsafe {
        printf(fmt_ptr, bytes_requested, time_taken);
    };
}
unsafe impl GlobalAlloc for ReportingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let start = std::time::Instant::now();
        let ptr = System.alloc(layout);
        let end = std::time::Instant::now();
        let time_taken = end - start;
        let bytes_requested = layout.size();
        log(bytes_requested, time_taken.as_nanos());
        ptr
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        System.dealloc(ptr, layout);
    }
}

struct World {
    current_turn: u64,
    particles: Vec<Particle>,
    height: f64,
    width: f64,
    rng: ThreadRng,
}

struct Particle {
    height: f64,
    width: f64,
    position: Vec2d<f64>,
    velocity: Vec2d<f64>,
    acceleration: Vec2d<f64>,
    color: [f32; 4],
}

impl Particle {
    fn new(world: &mut World) -> Particle {
        let x = world.rng.gen_range(0.0..=world.width);
        let y = world.height;
        let x_velocity = 0.0;
        let y_velocity = world.rng.gen_range(-2.0..0.0);
        let x_acceleration = 0.0;
        let y_acceleration = world.rng.gen_range(0.0..0.15);
        
        // Add color variation for visual appeal
        let r = world.rng.gen_range(0.8..=1.0);
        let g = world.rng.gen_range(0.6..=1.0);
        let b = world.rng.gen_range(0.4..=1.0);

        Particle {
            height: 4.0,
            width: 4.0,
            position: [x, y].into(),
            velocity: [x_velocity, y_velocity].into(),
            acceleration: [x_acceleration, y_acceleration].into(),
            color: [r, g, b, 0.99],
        }
    }

    fn update(&mut self) {
        self.velocity = add(self.velocity, self.acceleration);
        self.position = add(self.position, self.velocity);
        self.acceleration = mul_scalar(self.acceleration, 0.7);
        self.color[3] *= 0.995;
    }
    
    fn is_dead(&self) -> bool {
        self.color[3] < 0.02
    }
}

impl World {
    fn new(width: f64, height: f64) -> World {
        World {
            current_turn: 0,
            particles: Vec::with_capacity(1000),
            height,
            width,
            rng: thread_rng(),
        }
    }

    fn add_shapes(&mut self, n: i32) {
        for _ in 0..n.abs() {
            let particle = Particle::new(self);
            self.particles.push(particle);
        }
    }

    fn remove_shapes(&mut self, n: i32) {
        for _ in 0..n.abs() {
            if self.particles.is_empty() {
                break;
            }
            
            // Find first dead particle or remove oldest
            if let Some(i) = self.particles.iter().position(|p| p.is_dead()) {
                self.particles.swap_remove(i);
            } else {
                self.particles.swap_remove(0);
            }
        }
    }

    fn update(&mut self) {
        let n = self.rng.gen_range(-3..=3);

        if n > 0 {
            self.add_shapes(n);
        } else {
            self.remove_shapes(n);
        }

        // Remove dead particles in batch for efficiency
        self.particles.retain(|p| !p.is_dead());
        
        // Only shrink periodically to avoid frequent reallocations
        if self.current_turn % 100 == 0 {
            self.particles.shrink_to_fit();
        }
        
        for particle in &mut self.particles {
            particle.update();
        }
        self.current_turn += 1;
    }
}

fn main() {
    let (width, height) = (1920.0, 1080.0);
    let mut window: PistonWindow = WindowSettings::new("particles", [width, height])
        .exit_on_esc(true)
        .vsync(true)
        .build()
        .expect("Could not create a window.");

    let mut world = World::new(width, height);
    world.add_shapes(1000);

    while let Some(event) = window.next() {
        world.update();

        window.draw_2d(&event, |ctx, renderer, _device| {
            clear([0.15, 0.17, 0.17, 0.9], renderer);

            for particle in &world.particles {
                let size = [particle.position[0], particle.position[1], particle.width, particle.height];
                rectangle(particle.color, size, ctx.transform, renderer);
            }
        });
    }
}
