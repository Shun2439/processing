use nannou::prelude::*;

/* P.48 */
pub fn tulip(app: &App, frame: Frame) {
    // get canvas to draw on
    let draw = app.draw();

    draw.background().color(WHITE);

    /* petal */
    let draw = app.draw();
    draw.background().color(WHITE);

    /* petal */
    draw.ellipse()
        .xy(vec2(160.0, -70.0))
        .wh(vec2(50.0, 80.0))
        .stroke_weight(1.0); // ;, 0, PI);

    draw.rect()
        .xy(vec2(160.0, -45.0))
        .wh(vec2(55.0, 50.0))
        .color(WHITE);

    draw.line()
        .start(vec2(135.0, -70.0))
        .end(vec2(150.0, -80.0));
    draw.line()
        .start(vec2(150.0, -80.0))
        .end(vec2(160.0, -70.0));
    draw.line()
        .start(vec2(160.0, -70.0))
        .end(vec2(170.0, -80.0));
    draw.line()
        .start(vec2(170.0, -80.0))
        .end(vec2(185.0, -70.0));

    /* stem */
    draw.line()
        .start(vec2(160.0, -110.0))
        .end(vec2(160.0, -130.0));

    /* leaves */
    draw.ellipse()
        .xy(vec2(140.0, -120.0))
        .wh(vec2(20.0, 10.0))
        .stroke_weight(1.0);
    draw.ellipse()
        .xy(vec2(180.0, -120.0))
        .wh(vec2(20.0, 10.0))
        .stroke_weight(1.0);

    /* pod */
    draw.quad()
        .points(
            vec2(130.0, -130.0),
            vec2(140.0, -160.0),
            vec2(180.0, -160.0),
            vec2(190.0, -130.0),
        )
        .stroke_weight(1.0);

    // draw.ellipse().color(WHITE).width(200.0).height(100.0);

    // put everything on the frame
    draw.to_frame(app, &frame).unwrap();
}
