use nannou::prelude::*;
use palette::named;

use crate::model::Model;
use crate::settings::*;
use crate::sorting::SortingStateEnum;


fn draw_hood(draw: &Draw, win: Rect<f32>, model: &Model) {
    let hood = Rect::from_w_h(win.w(), win.h() * 0.18)
        .top_left_of(Rect::from(win));
    draw.rect().xy(hood.xy()).wh(hood.wh())
        .color(named::PURPLE);
    draw.text("Press 'H' for help").xy(hood.top_left() + vec2(70.0, -10.0))
        .font_size(16)
        .color(named::WHITE);
    draw.text(model.selected_sort.to_string().as_str())
        .xy(hood.xy())
        .font_size(32)
        .color(named::WHITE);
}

fn draw_sorting_states(draw: &Draw, win: Rect<f32>, model: &Model) {
    let width_of_rect: f32 = win.w() / model.n as f32;
    let height_of_rect: f32 = (win.h() - (win.h() * 0.2)) / model.n as f32;
    for (i, val) in model.sorter.get_states()[model.index].iter().enumerate() {
        let r = Rect::from_w_h(width_of_rect, height_of_rect * (val.value as f32))
            .bottom_left_of(Rect::from(win).shift_x(width_of_rect * i as f32))
            .pad(1.0);
        draw.rect().xy(r.xy()).wh(r.wh()).color(
            match val.state {
                SortingStateEnum::FREE => named::DARKGREY,
                SortingStateEnum::COMPARE => named::LIGHTGREEN,
                SortingStateEnum::SWAP => named::TOMATO,
                SortingStateEnum::COMPLETED => named::GREEN,
                SortingStateEnum::CHECKING => named::LIGHTBLUE,
            }
        );
    }
}

fn draw_help(draw: &Draw, win: Rect<f32>, model: &Model) {
    if model.show_help {
        let help = Rect::from_w_h(win.w() * 0.7, win.h() * 0.7);
        draw.rect().xy(help.xy()).wh(help.wh()).color(named::WHITE);
        draw.text(HELP_TEXT)
            .xy(help.xy())
            .font_size(16)
            .color(named::BLACK);
    }
}

pub fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(named::ALICEBLUE);

    let win: Rect<f32> = app.window_rect();

    draw_hood(&draw, win, model);
    draw_sorting_states(&draw, win, model);
    draw_help(&draw, win, model);

    draw.to_frame(app, &frame).unwrap();
}
