use nannou::prelude::*;

mod osaka;

fn main() {
    nannou::app(model).run();
}

fn model(app: &App) {
    app.new_window().size(640, 640).view(view).build().unwrap();
}

fn view(app: &App, _model: &(), frame: Frame) {
    osaka::osaka(app, frame);
}
