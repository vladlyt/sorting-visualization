use crate::sorting::{Sorter, SortingState, SortingValue, SortModel};

pub struct QuickSort {}

impl QuickSort {
    pub fn new() -> Self {
        Self {}
    }

    pub fn partition(&self, sorter: &mut SortModel, low: usize, high: usize) -> usize {
        let mut i = low;
        let pivot = sorter.get_current_state()[high];
        sorter.set_left_right(low, high);

        for j in low..high {
            sorter.set_left_right(i, high);
            sorter.compare_index(j);
            if sorter.get_current_state()[j] <= pivot {
                sorter.swap(i, j);
                i += 1;
            }
        }
        sorter.swap(i, high);
        sorter.unset_left_right();
        i
    }

    fn quicksort(&self, sorter: &mut SortModel, low: usize, high: usize) {
        if sorter.get_current_state().len() <= 1 {
            return;
        }

        if low < high {
            let pi = self.partition(sorter, low, high);
            if pi > 0 {
                self.quicksort(sorter, low, pi - 1);
            }
            self.quicksort(sorter, pi + 1, high);
        }
    }
}

impl Sorter for QuickSort {
    fn sort(&mut self, values: Vec<u32>) -> SortModel {
        let mut sorter = SortModel::new(values);

        let len = sorter.len();
        if len >= 2 {
            self.quicksort(&mut sorter, 0, len - 1);
        }
        sorter.complete();

        sorter
    }
}


#[cfg(test)]
pub mod tests {
    use crate::*;
    use crate::utils::*;

    #[test]
    fn quicksort_unsorted_test() {
        let mut to_sort_slice = vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1];

        let sorted_model = QuickSort::new().sort(to_sort_slice);

        assert!(is_sorted(sorted_model.get_final_state()));
    }

    #[test]
    fn quicksort_sorted_test() {
        let mut to_sort_slice = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

        let sorted_model = QuickSort::new().sort(to_sort_slice);

        assert!(is_sorted(sorted_model.get_final_state()));
    }

    #[test]
    fn quicksort_empty_test() {
        let mut to_sort_slice = vec![];

        let sorted_model = QuickSort::new().sort(to_sort_slice);

        assert_eq!(sorted_model.get_states().len(), 1);
        assert_eq!(sorted_model.get_final_state().len(), 0);
    }

    #[test]
    fn quicksort_random_test() {
        for _ in 0..10 {
            let mut to_sort_slice = random_vec(30);

            let sorted_model = QuickSort::new().sort(to_sort_slice);

            assert!(is_sorted(sorted_model.get_final_state()));
        }
    }
}