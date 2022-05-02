use crate::sorting::{SortModel, SortingState, Sorter, SortingValue};


pub struct QuickSort {
    sorter: SortModel,
}

impl QuickSort {
    pub fn new(v: Vec<i32>) -> Self {
        Self {
            sorter: SortModel::new(v),
        }
    }

    pub fn partition(&mut self, low: usize, high: usize) -> usize {
        let mut i = low;
        let pivot = self.sorter.current_state[high];
        self.sorter.set_left_right(low, high);

        for j in low..high {
            self.sorter.set_left_right(i, high);
            self.sorter.compare_index(j);
            if self.sorter.current_state[j] <= pivot {
                self.sorter.swap_values(i, j);
                i += 1;
            }
        }
        self.sorter.swap_values(i, high);
        self.sorter.unset_left_right();
        i
    }

    fn quicksort(&mut self, low: usize, high: usize) {
        if self.sorter.current_state.len() <= 1 {
            return;
        }

        if low < high {
            let pi = self.partition(low, high);
            if pi > 0 {
                self.quicksort(low, pi - 1);
            }
            self.quicksort(pi + 1, high);
        }
    }
}

impl Sorter for QuickSort {
    fn sort(&mut self) -> &mut Vec<SortingState> {
        self.quicksort(0, self.sorter.current_state.len() - 1);
        self.sorter.complete();
        &mut self.sorter.states
    }
}

