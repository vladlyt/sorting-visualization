use std::fmt;
use std::slice::Iter;

use nannou::prelude::*;
use nannou::winit::event::VirtualKeyCode;

use crate::bubble_sort::BubbleSort;
use crate::insertion_sort::InsertionSort;
use crate::merge_sort::MergeSort;
use crate::model::Model;
use crate::quicksort::QuickSort;
use crate::settings::*;
use crate::sorting::{Sorter, SortingState, SortModel};

#[derive(Debug, Copy, Clone)]
pub enum SortsEnum {
    BubbleSort,
    MergeSort,
    QuickSort,
    InsertionSort,
}


impl fmt::Display for SortsEnum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SortsEnum::BubbleSort => write!(f, "Bubble Sort"),
            SortsEnum::MergeSort => write!(f, "Merge Sort"),
            SortsEnum::QuickSort => write!(f, "Quick Sort"),
            SortsEnum::InsertionSort => write!(f, "Insertion Sort"),
        }
    }
}

impl SortsEnum {
    pub fn get_next(&self) -> SortsEnum {
        match self {
            SortsEnum::BubbleSort => SortsEnum::MergeSort,
            SortsEnum::MergeSort => SortsEnum::QuickSort,
            SortsEnum::QuickSort => SortsEnum::InsertionSort,
            SortsEnum::InsertionSort => SortsEnum::BubbleSort,
        }
    }

    pub fn get_prev(&self) -> SortsEnum {
        match self {
            SortsEnum::BubbleSort => SortsEnum::InsertionSort,
            SortsEnum::MergeSort => SortsEnum::BubbleSort,
            SortsEnum::QuickSort => SortsEnum::MergeSort,
            SortsEnum::InsertionSort => SortsEnum::QuickSort,
        }
    }

    pub fn get_sorter(&self) -> Box<dyn Sorter> {
        match self {
            SortsEnum::BubbleSort => Box::new(BubbleSort::new()),
            SortsEnum::MergeSort => Box::new(MergeSort::new()),
            SortsEnum::QuickSort => Box::new(QuickSort::new()),
            SortsEnum::InsertionSort => Box::new(InsertionSort::new()),
        }
    }

    pub fn get_sorter_from_state(&self, state: &SortingState) -> SortModel {
        self.get_sorter().sort(state.into_iter().map(|x| x.value).collect())
    }

    #[allow(dead_code)]
    pub fn iter() -> Iter<'static, Self> {
        [
            SortsEnum::BubbleSort,
            SortsEnum::InsertionSort,
            SortsEnum::QuickSort,
            SortsEnum::MergeSort,
        ].iter()
    }
}

pub fn event(_app: &App, model: &mut Model, event: WindowEvent) {
    match event {
        KeyPressed(_key) => {
            println!("{:?}", event);
            match _key {
                VirtualKeyCode::Right => {
                    if model.index + 1 < model.sorter.get_states().len() {
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
                    model.change_selected_sort(&model.selected_sort.get_next());
                }
                VirtualKeyCode::Down => {
                    model.change_selected_sort(&model.selected_sort.get_prev());
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

#[cfg(test)]
pub mod tests {
    use crate::utils::*;

    use super::*;

    #[test]
    fn unsorted_test() {
        let to_sort_slice = vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1];

        for sort in SortsEnum::iter() {
            println!("Sorting: {}", sort);
            let sorted_model = sort.get_sorter().sort(to_sort_slice.clone());
            assert!(is_sorted(sorted_model.get_final_state()));
        }
    }

    #[test]
    fn sorted_test() {
        let to_sort_slice = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

        for sort in SortsEnum::iter() {
            println!("Sorting: {}", sort);
            let sorted_model = sort.get_sorter().sort(to_sort_slice.clone());
            assert!(is_sorted(sorted_model.get_final_state()));
        }
    }

    #[test]
    fn empty_test() {
        let to_sort_slice = vec![];

        for sort in SortsEnum::iter() {
            println!("Sorting: {}", sort);
            let sorted_model = sort.get_sorter().sort(to_sort_slice.clone());
            assert_eq!(sorted_model.get_states().len(), 1);
            assert_eq!(sorted_model.get_final_state().len(), 0);
        }
    }

    #[test]
    fn single_value_test() {
        let to_sort_slice = vec![10];

        for sort in SortsEnum::iter() {
            println!("Sorting: {}", sort);
            let sorted_model = sort.get_sorter().sort(to_sort_slice.clone());
            assert_eq!(sorted_model.get_states().len(), 2);
            assert_eq!(sorted_model.get_final_state().len(), 1);
        }
    }

    #[test]
    fn random_test() {
        for _ in 0..10 {
            let to_sort_slice = random_vec(30);

            for sort in SortsEnum::iter() {
                println!("Sorting: {}", sort);
                let sorted_model = sort.get_sorter().sort(to_sort_slice.clone());
                assert!(is_sorted(sorted_model.get_final_state()));
            }
        }
    }
}
