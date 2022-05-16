use crate::settings::*;
use crate::controller::SortsEnum;
use crate::sorting::{SortingState, SortModel};
use crate::utils;

pub struct Model {
    pub sorter: SortModel,
    pub selected_sort: SortsEnum,
    pub index: usize,
    pub n: u32,
    pub keep_rolling: bool,
    pub show_help: bool,
}

impl Model {

    pub fn default() -> Self {
        Model::new(&SortsEnum::BubbleSort)
    }

    pub fn new(sort_kind: &SortsEnum) -> Self {
        Model::from_values(
            &sort_kind,
            &utils::shuffled_sorting_state(START_NUMBER),
        )
    }

    pub fn from_values(sort_kind: &SortsEnum, states: &SortingState) -> Self {
        Model {
            sorter: sort_kind.get_sorter_from_state(states),
            selected_sort: sort_kind.clone(),
            index: 0,
            keep_rolling: false,
            n: states.len() as u32,
            show_help: false,
        }
    }

    pub fn change_selected_sort(&mut self, sort_kind: &SortsEnum) {
        *self = Model::from_values(
            &sort_kind,
            &self.sorter.get_states()[self.index],
        );
    }

    pub fn change_n(&mut self, n: u32) {
        *self = Model::from_values(
            &self.selected_sort,
            &utils::shuffled_sorting_state(n),
        );
    }
}