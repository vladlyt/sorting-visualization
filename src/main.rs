use nannou::prelude::*;

use crate::controller::event as event_fn;
use crate::model::Model;
use crate::view::view as view_fn;

mod sorting;
mod bubble_sort;
mod merge_sort;
mod insertion_sort;
mod quicksort;
mod utils;
mod controller;
mod view;
mod model;
mod settings;


// TODO add README with video or gif
// TODO add other sorting algorithms
// TODO move to sorts package

fn model(app: &App) -> Model {
    app.new_window()
        .event(event_fn)
        .view(view_fn)
        .build()
        .unwrap();
    Model::default()
}


fn update(_app: &App, model: &mut Model, _update: Update) {
    if model.keep_rolling {
        if model.index + 1 < model.sorter.get_states().len() {
            model.index += 1;
        }
    }
}


fn main() {
    nannou::app(model).update(update).run();
}
