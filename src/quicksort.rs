use crate::sorting::{Sorter, SortingState, SortingValue, SortModel};

pub struct QuickSort {
    sorter: SortModel,
}

impl QuickSort {
    pub fn new(v: Vec<u32>) -> Self {
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
        if self.sorter.current_state.len() != 0 {
            self.quicksort(0, self.sorter.current_state.len() - 1);
            self.sorter.complete();
        }
        &mut self.sorter.states
    }
}


#[cfg(test)]
pub mod tests {
    use crate::*;
    use crate::utils::*;

    #[test]
    fn quick_sort_unsorted_test() {
        let mut to_sort_slice = vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1];

        let mut sorter = QuickSort::new(to_sort_slice);
        let states = sorter.sort();

        assert!(states.len() > 0);
        assert!(is_sorted(&states[states.len() - 1]));
    }

    #[test]
    fn quick_sort_sorted_test() {
        let mut to_sort_slice = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

        let mut sorter = QuickSort::new(to_sort_slice);
        let states = sorter.sort();

        assert!(states.len() > 0);
        assert!(is_sorted(&states[states.len() - 1]));
    }

    #[test]
    fn quick_sort_empty_test() {
        let mut to_sort_slice = vec![];

        let mut sorter = QuickSort::new(to_sort_slice);
        let states = sorter.sort();

        assert!(states.len() == 0);
    }

    #[test]
    fn quick_sort_random_test() {
        for _ in 0..10 {
            let mut to_sort_slice = random_vec(30);
            let mut sorter = QuickSort::new(to_sort_slice);
            let states = sorter.sort();

            assert!(states.len() > 0);
            assert!(is_sorted(&states[states.len() - 1]));
        }
    }
}

