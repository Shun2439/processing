use nannou::prelude::*;

mod fire_horse;

use fire_horse::Model;

fn main() {
    nannou::app(model).update(update).run();
}

fn model(app: &App) -> Model {
    app.new_window().size(800, 600).view(view).build().unwrap();

    Model::new()
}

fn update(app: &App, model: &mut Model, _update: Update) {
    model.update(app);
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(BLACK);

    model.draw(&draw);

    draw.to_frame(app, &frame).unwrap();
}
