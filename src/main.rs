use std::cmp::Ordering;
use nannou::prelude::*;
use std::thread::sleep;
use std::time::Duration;
use nannou::winit::event::VirtualKeyCode;
use palette::named;
use rand::prelude::SliceRandom;
use crate::sorting::Sorter;
use crate::bubble_sort::BubbleSort;
use crate::merge_sort::MergeSort;
use crate::quicksort::QuickSort;

mod lib;
mod sorting;
mod bubble_sort;
mod merge_sort;
mod quicksort;

const N: i32 = 50;


struct Model {
    states: Vec<sorting::SortingState>,
    index: usize,
    keep_rolling: bool,
}


fn event(_app: &App, model: &mut Model, event: WindowEvent) {
    println!("{:?}", event);
    match event {
        KeyPressed(_key) => {
            match _key {
                VirtualKeyCode::Right => {
                    if model.index + 1 < model.states.len() {
                        model.index += 1;
                    }
                    model.keep_rolling = false;
                }
                VirtualKeyCode::Left => {
                    if model.index > 0 {
                        model.index -= 1;
                    }
                    model.keep_rolling = false;
                }
                VirtualKeyCode::Space => {
                    model.keep_rolling = !model.keep_rolling;
                }
                _ => {}
            }
        }
        KeyReleased(_key) => {}

        // Mouse events
        MouseMoved(_pos) => {}
        MousePressed(_button) => {}
        MouseReleased(_button) => {}
        MouseWheel(_amount, _phase) => {}
        MouseEntered => {}
        MouseExited => {}

        // Touch events
        Touch(_touch) => {}
        TouchPressure(_pressure) => {}

        // Window events
        Moved(_pos) => {}
        Resized(_size) => {}
        HoveredFile(_path) => {}
        DroppedFile(_path) => {}
        HoveredFileCancelled => {}
        Focused => {}
        Unfocused => {}
        Closed => {}
    }
}

fn model(app: &App) -> Model {
    app.new_window().event(event).view(view).build().unwrap();

    let states = QuickSort::new(lib::tests::shuffled_vec(N)).sort().to_vec();

    Model {
        states,
        index: 0,
        keep_rolling: false,
    }
}


fn update(app: &App, model: &mut Model, _update: Update) {
    if model.keep_rolling {
        if model.index + 1 < model.states.len() {
            model.index += 1;
        }
    }
}


fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(named::ALICEBLUE);

    let win: Rect<f32> = app.window_rect();

    let width_of_rect: f32 = win.w() / N as f32;
    let height_of_rect: f32 = win.h() / N as f32;

    for (i, val) in model.states[model.index].iter().enumerate() {
        let r = Rect::from_w_h(width_of_rect, height_of_rect * (val.value as f32))
            .bottom_left_of(Rect::from(win).shift_x(width_of_rect * i as f32))
            .pad(1.0);

        // println!("{:#?} {:#?}", r, bottom_left_rect);
        draw.rect().xy(r.xy()).wh(r.wh()).color(
            match val.state {
                sorting::SortingStateEnum::FREE => named::DARKGREY,
                sorting::SortingStateEnum::COMPARE => named::LIGHTGREEN,
                sorting::SortingStateEnum::SWAP => named::TOMATO,
                sorting::SortingStateEnum::LEFT => named::YELLOW,
                sorting::SortingStateEnum::RIGHT => named::PURPLE,
            }
        );
    }

    draw.to_frame(app, &frame).unwrap();
}

fn main() {
    nannou::app(model).update(update).run();
}
