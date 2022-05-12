use std::cmp::Ordering;
use std::fmt;
use std::thread::sleep;
use std::time::Duration;

use nannou::glam::Vec2Swizzles;
use nannou::prelude::*;
use nannou::winit::event::VirtualKeyCode;
use palette::named;
use rand::prelude::SliceRandom;
use rand::Rng;

use crate::bubble_sort::BubbleSort;
use crate::merge_sort::MergeSort;
use crate::quicksort::QuickSort;
use crate::sorting::{Sorter, SortingState};

mod sorting;
mod bubble_sort;
mod merge_sort;
mod quicksort;
mod utils;

const MIN_NUMBER: u32 = 10;
const MAX_NUMBER: u32 = 100;
const START_NUMBER: u32 = 40;
const STEP_SIZE: u32 = 5;
const HELP_TEXT: &str = "
    Welcome to the sorting visualizer!\n
    Use the UP and DOWN keys to change the sorting algorithm.\n
    Use the LEFT and RIGHT keys to step the sorting states.\n
    Use the SPACE to stop/run sorting algorithm.\n
    Use the LEFT and RIGHT brackets to change the number of elements.\n
    Use the 'R' key regenerate numbers in array.\n
    Use the 'H' key to toggle this help text.\n
    Use the 'Q' key to quit.
    ";

enum SortsEnum {
    BubbleSort,
    MergeSort,
    QuickSort,
}


impl fmt::Display for SortsEnum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SortsEnum::BubbleSort => write!(f, "Bubble Sort"),
            SortsEnum::MergeSort => write!(f, "Merge Sort"),
            SortsEnum::QuickSort => write!(f, "Quick Sort"),
        }
    }
}

impl SortsEnum {
    fn get_next(&self) -> SortsEnum {
        match self {
            SortsEnum::BubbleSort => SortsEnum::MergeSort,
            SortsEnum::MergeSort => SortsEnum::QuickSort,
            SortsEnum::QuickSort => SortsEnum::BubbleSort,
        }
    }

    fn get_prev(&self) -> SortsEnum {
        match self {
            SortsEnum::BubbleSort => SortsEnum::QuickSort,
            SortsEnum::MergeSort => SortsEnum::BubbleSort,
            SortsEnum::QuickSort => SortsEnum::MergeSort,
        }
    }

    fn get_sorted_states(&self, n: u32) -> Vec<SortingState> {
        match self {
            SortsEnum::BubbleSort => { BubbleSort::new(utils::shuffled_vec(n)).sort().to_vec() }
            SortsEnum::MergeSort => { MergeSort::new(utils::shuffled_vec(n)).sort().to_vec() }
            SortsEnum::QuickSort => { QuickSort::new(utils::shuffled_vec(n)).sort().to_vec() }
        }
    }
}


struct Model {
    states: Vec<SortingState>,
    index: usize,
    keep_rolling: bool,
    n: u32,
    selected_sort: SortsEnum,
    show_help: bool,
}

impl Model {
    fn change_sort(&mut self, sort_kind: SortsEnum, n: u32) {
        self.selected_sort = sort_kind;
        self.change_n(n);
    }

    fn change_n(&mut self, n: u32) {
        self.index = 0;
        self.n = n;
        self.states = self.selected_sort.get_sorted_states(n);
        self.keep_rolling = false;
    }
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
                VirtualKeyCode::Up => {
                    model.change_sort(model.selected_sort.get_next(), model.n);
                }
                VirtualKeyCode::Down => {
                    model.change_sort(model.selected_sort.get_prev(), model.n);
                }
                VirtualKeyCode::Space => {
                    model.keep_rolling = !model.keep_rolling;
                }
                VirtualKeyCode::RBracket => {
                    if model.n + STEP_SIZE < MAX_NUMBER {
                        model.change_n(model.n + STEP_SIZE);
                    } else {
                        model.change_n(MAX_NUMBER);
                    }
                }
                VirtualKeyCode::LBracket => {
                    if model.n - STEP_SIZE > MIN_NUMBER {
                        model.change_n(model.n - STEP_SIZE);
                    } else {
                        model.change_n(MIN_NUMBER);
                    }
                }
                VirtualKeyCode::R => {
                    model.change_n(model.n);
                }
                VirtualKeyCode::H => {
                    model.show_help = !model.show_help;
                }
                VirtualKeyCode::Q => {
                    std::process::exit(0);
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

    Model {
        states: SortsEnum::BubbleSort.get_sorted_states(START_NUMBER),
        selected_sort: SortsEnum::BubbleSort,
        index: 0,
        keep_rolling: false,
        n: START_NUMBER,
        show_help: false,
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

    let width_of_rect: f32 = win.w() / model.n as f32;
    let height_of_rect: f32 = (win.h() - (win.h() * 0.2)) / model.n as f32;

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

    if model.show_help {
        let help = Rect::from_w_h(win.w() * 0.7, win.h() * 0.7);
        draw.rect().xy(help.xy()).wh(help.wh()).color(named::WHITE);
        draw.text(HELP_TEXT)
            .xy(help.xy())
            .font_size(16)
            .color(named::BLACK);
    }

    draw.to_frame(app, &frame).unwrap();
}

fn main() {
    nannou::app(model).update(update).run();
}
