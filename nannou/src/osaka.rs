use nannou::{prelude::*, rand::{rand, Rng}};

/* https://scrapbox.io/prog-exercises/EXPO2025 */

pub struct Spot {
    pub deg: f32,
    pub dx: f32,
    pub dy: f32,
    pub r: f32,
    pub r2: f32,
    pub vx: f32,
    pub vy: f32,
}

impl Spot {
    pub fn new(deg: f32, r: f32) -> Self {
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

pub fn draw_spot(app: &App, spot: &mut Spot) {
    let width = app.window_rect().w();
    let height = app.window_rect().h();

    let ox = spot.deg.to_radians().cos() * width / 3.0;
    let oy = spot.deg.to_radians().sin() * height / 3.0;

    let draw = app.draw();

    draw.ellipse()
        .xy(vec2(ox + spot.dx, oy + spot.dy))
        .wh(vec2(spot.r as f32, spot.r2 as f32))
        .color(RED);
}

pub fn move_spot(spot: &mut Spot) {
    let mut rng = rand::thread_rng();
    // JavaScriptのロジックに合わせて、vxとvyを毎回計算しなおします。
    // 乱数も浮動小数点数の範囲で生成します。
    spot.vx = rng.gen_range(-40.0..=40.0) / 10.0 - spot.dx / 2.0;
    spot.vy = rng.gen_range(-40.0..=40.0) / 10.0 - spot.dy / 2.0;
    // 計算した速度で位置(dx, dy)を更新します。
    spot.dx += spot.vx;
    spot.dy += spot.vy;
    spot.deg += 0.3; // 角度を更新
}

pub fn osaka(app: &App, frame: Frame) {

    let mut spots = Vec::new();

    for mut i in 0..360 {
        let r = rand::thread_rng().gen_range(30.0..=150.0);
        spots.push(Spot::new(i as f32, r));
        i += (r / 4.0) as i32;
    }
    // get canvas to draw on
    let draw = app.draw();

    draw.background().color(BLACK);

    spots.iter_mut().for_each(|spot| {
        draw_spot(app, spot);
        move_spot(spot);
    });

    // put everything on the frame
    draw.to_frame(app, &frame).unwrap();
}
