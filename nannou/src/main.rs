use nannou::prelude::*;

mod tulip;

fn main() {
    nannou::app(model).run();
}

fn model(app: &App) {
    app.new_window().size(512, 512).view(view).build().unwrap();
}

fn view(app: &App, _model: &(), frame: Frame) {
    tulip::tulip(app, frame);
}