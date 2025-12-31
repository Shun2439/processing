use nannou::{
    prelude::*,
    rand::{Rng, rand},
};

pub struct Particle {
    pub pos: Point2,
    pub vel: Vec2,
    pub life: f32, // 1.0 (new) to 0.0 (dead)
    pub size: f32,
    pub color: Hsva,
}

impl Particle {
    pub fn new(pos: Point2) -> Self {
        let mut rng = rand::thread_rng();
        let vx = rng.gen_range(-0.5..=0.5);
        let vy = rng.gen_range(1.0..=3.0);

        Particle {
            pos,
            vel: vec2(vx, vy),
            life: 1.0,
            size: random_range(10.0, 30.0),
            color: hsva(0.1, 0.8, 1.0, 1.0), // Start with yellowish
        }
    }

    pub fn update(&mut self) {
        self.pos += self.vel;
        self.life -= 0.015; // Particle dies over time

        // Add some jitter
        self.vel.x += random_range(-0.1, 0.1);
        self.vel.y += 0.05; // Upward acceleration

        // Update color and size based on life
        // 1.0 (Yellow) -> 0.5 (Orange/Red) -> 0.0 (Gray/Fade)
        let hue = 0.0 + (self.life * 0.15); // Shift from red towards yellow
        let sat = 0.9;
        let val = self.life; // Fade out brightness
        self.color = hsva(hue, sat, val, self.life);

        self.size *= 0.98; // Slightly shrink
    }

    pub fn is_dead(&self) -> bool {
        self.life <= 0.0
    }
}

pub struct Model {
    pub particles: Vec<Particle>,
}

impl Model {
    pub fn new() -> Self {
        Model {
            particles: Vec::new(),
        }
    }

    pub fn update(&mut self, _app: &App) {
        // Spawn new particles at the base of the fire
        for _ in 0..5 {
            let base_pos = pt2(random_range(-20.0, 20.0), -150.0);
            self.particles.push(Particle::new(base_pos));
        }

        // Update existing particles
        for p in &mut self.particles {
            p.update();
        }

        // Remove dead particles
        self.particles.retain(|p| !p.is_dead());
    }

    pub fn draw(&self, draw: &Draw) {
        // Use additive blending for the "glow" effect
        let draw = draw.color_blend(BLEND_ADD);

        for p in &self.particles {
            draw.ellipse()
                .xy(p.pos)
                .wh(vec2(p.size, p.size * 1.5)) // Slightly elongated
                .color(p.color);
        }

        // Add a central glow
        draw.ellipse()
            .xy(pt2(0.0, -150.0))
            .wh(vec2(60.0, 40.0))
            .color(hsva(0.1, 0.7, 0.3, 0.1));
    }
}
