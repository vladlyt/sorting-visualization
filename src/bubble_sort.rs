use crate::sorting::{Sorter, SortingState, SortModel};

pub struct BubbleSort {}

impl BubbleSort {
    pub fn new() -> Self {
        Self {}
    }
}

impl Sorter for BubbleSort {
    fn sort(&mut self, values: Vec<u32>) -> SortModel {
        let mut sorter = SortModel::new(values);

        let len = sorter.get_current_state().len();
        for i in 0..len {
            for j in 0..(len - i - 1) {
                if sorter.left_is_greater(j, j + 1) {
                    sorter.swap(j, j + 1);
                }
            }
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
    fn bubble_sort_unsorted_test() {
        let mut to_sort_slice = vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1];

        let sorted_model = BubbleSort::new().sort(to_sort_slice);

        assert!(is_sorted(sorted_model.get_final_state()));
    }

    #[test]
    fn bubble_sort_sorted_test() {
        let mut to_sort_slice = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

        let sorted_model = BubbleSort::new().sort(to_sort_slice);

        assert!(is_sorted(sorted_model.get_final_state()));
    }

    #[test]
    fn bubble_sort_empty_test() {
        let mut to_sort_slice = vec![];

        let sorted_model = BubbleSort::new().sort(to_sort_slice);

        assert_eq!(sorted_model.get_states().len(), 1);
        assert_eq!(sorted_model.get_final_state().len(), 0);
    }

    #[test]
    fn bubble_sort_random_test() {
        for _ in 0..10 {
            let mut to_sort_slice = random_vec(30);

            let sorted_model = BubbleSort::new().sort(to_sort_slice);

            assert!(is_sorted(sorted_model.get_final_state()));
        }
    }
}
