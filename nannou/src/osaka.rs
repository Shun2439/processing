use nannou::{image::imageops, prelude::*, rand::{rand, Rng}};

/* https://scrapbox.io/prog-exercises/EXPO2025 */

pub struct Spot {
    pub deg: f64,
    pub dx: f64,
    pub dy: f64,
    pub r: f64,
    pub r2: f64,
    pub vx: f64,
    pub vy: f64,
}

impl Spot {
    pub fn new(deg: f64, r: f64) -> Self {
        let mut rng = rand::thread_rng();

        Spot {
            deg,
            dx: rng.gen_range(-10.0..=10.0),
            dy: rng.gen_range(-10.0..=10.0),
            r,
            r2: r * rng.gen_range(0.5..=1.5),
            vx: rng.gen_range(-2.0..=2.0),
            vy: rng.gen_range(-2.0..=2.0),
        }
    }
}

pub fn osaka(app: &App, frame: Frame) {

    let mut spots = Vec::new();

    for mut i in 0..360 {
        let r = rand::thread_rng().gen_range(30.0..=150.0);
        spots.push(Spot::new(i as f64, r));
        i += (r / 4.0) as i32;
    }
    // get canvas to draw on
    let draw = app.draw();

    draw.background().color(BLACK);

    // put everything on the frame
    draw.to_frame(app, &frame).unwrap();
}
