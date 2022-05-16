use nannou::prelude::*;

use crate::controller::event as event_fn;
use crate::model::Model;
use crate::view::view as view_fn;

mod controller;
mod model;
mod settings;
mod utils;
mod view;

// TODO add README with gif
// TODO add built binary to github
// TODO add other sorting algorithms

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
