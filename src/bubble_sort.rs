use crate::sorting::{Sorter, SortingState, SortModel};

pub struct BubbleSort {
    sorter: SortModel,
}

impl BubbleSort {
    pub fn new(v: Vec<u32>) -> Self {
        Self {
            sorter: SortModel::new(v),
        }
    }
}

impl Sorter for BubbleSort {
    fn sort(&mut self) -> &mut Vec<SortingState> {
        for i in 0..self.sorter.current_state.len() {
            for j in 0..(self.sorter.current_state.len() - i - 1) {
                if self.sorter.value_is_greater(j, j + 1) {
                    self.sorter.swap_values(j, j + 1);
                }
            }
        }
        self.sorter.complete();
        &mut self.sorter.states
    }
}

#[cfg(test)]
pub mod tests {
    use crate::*;
    use crate::utils::*;

    #[test]
    fn bubble_sort_unsorted_test() {
        let mut to_sort_slice = vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1];

        let mut sorter = BubbleSort::new(to_sort_slice);
        let states = sorter.sort();

        assert!(states.len() > 0);
        assert!(is_sorted(&states[states.len() - 1]));
    }

    #[test]
    fn bubble_sort_sorted_test() {
        let mut to_sort_slice = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

        let mut sorter = BubbleSort::new(to_sort_slice);
        let states = sorter.sort();

        assert!(states.len() > 0);
        assert!(is_sorted(&states[states.len() - 1]));
    }

    #[test]
    fn bubble_sort_empty_test() {
        let mut to_sort_slice = vec![];

        let mut sorter = BubbleSort::new(to_sort_slice);
        let states = sorter.sort();

        assert!(states.len() == 0);
    }

    #[test]
    fn bubble_sort_random_test() {
        for _ in 0..10 {
            let mut to_sort_slice = random_vec(30);
            let mut sorter = BubbleSort::new(to_sort_slice);
            let states = sorter.sort();

            assert!(states.len() > 0);
            assert!(is_sorted(&states[states.len() - 1]));
        }
    }
}
