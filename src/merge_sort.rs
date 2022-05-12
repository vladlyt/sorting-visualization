use crate::sorting::{SortModel, SortingState, Sorter, SortingValue};

pub struct MergeSort {
    sorter: SortModel,
}

impl MergeSort {
    pub fn new(v: Vec<u32>) -> Self {
        Self {
            sorter: SortModel::new(v),
        }
    }

    fn merge_sort(&mut self) {
        if self.sorter.current_state.len() <= 1 {
            return;
        }

        let mut left: Vec<u32> = self.sorter.current_state[0..self.sorter.current_state.len() / 2]
            .iter()
            .map(|v| v.value)
            .collect();
        let mut right: Vec<u32> = self.sorter.current_state[self.sorter.current_state.len() / 2..self.sorter.current_state.len()]
            .iter()
            .map(|v| v.value)
            .collect();

        let right_backup: SortingState = right.clone().iter().map(|v| SortingValue::new(*v)).collect();

        let mut left_sort = MergeSort::new(left);
        let mut right_sort = MergeSort::new(right);

        left_sort.merge_sort();
        right_sort.merge_sort();
        // TODO rewrite
        self.sorter.states.extend(
            left_sort.sorter.states
                .iter()
                .map(
                    |state| [&state[..], &right_backup[..]].concat()
                ).collect::<Vec<SortingState>>()
        );
        self.sorter.states.extend(
            right_sort.sorter.states
                .iter()
                .map(
                    |state| [&left_sort.sorter.current_state[..], &state[..]].concat()
                ).collect::<Vec<SortingState>>()
        );

        let left_len = left_sort.sorter.current_state.len();
        let right_len = right_sort.sorter.current_state.len();

        self.sorter.current_state = [&left_sort.sorter.current_state[..], &right_sort.sorter.current_state[..]].concat();

        let (mut i, mut j, mut k) = (0, 0, 0);
        while i < left_len && j < right_len {
            self.sorter.compare_index(k);
            if left_sort.sorter.current_state[i] < right_sort.sorter.current_state[j] {
                self.sorter.set_value(k, left_sort.sorter.current_state[i]);
                i += 1;
            } else {
                self.sorter.set_value(k, right_sort.sorter.current_state[j]);
                j += 1;
            }
            k += 1;
        }

        while i < left_len {
            // self.sorter.compare_index(k);
            self.sorter.set_value(k, left_sort.sorter.current_state[i]);
            i += 1;
            k += 1;
        }

        while j < right_len {
            // self.sorter.compare_index(k);
            self.sorter.set_value(k, right_sort.sorter.current_state[j]);
            j += 1;
            k += 1;
        }
    }
}

impl Sorter for MergeSort {
    fn sort(&mut self) -> &mut Vec<SortingState> {
        self.merge_sort();
        self.sorter.complete();
        &mut self.sorter.states
    }
}


#[cfg(test)]
pub mod tests {
    use crate::*;
    use crate::utils::*;

    #[test]
    fn merge_sort_unsorted_test() {
        let mut to_sort_slice = vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1];

        let mut sorter = MergeSort::new(to_sort_slice);
        let states = sorter.sort();

        assert!(states.len() > 0);
        assert!(is_sorted(&states[states.len() - 1]));
    }

    #[test]
    fn merge_sort_sorted_test() {
        let mut to_sort_slice = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

        let mut sorter = MergeSort::new(to_sort_slice);
        let states = sorter.sort();

        assert!(states.len() > 0);
        assert!(is_sorted(&states[states.len() - 1]));
    }

    #[test]
    fn merge_sort_empty_test() {
        let mut to_sort_slice = vec![];

        let mut sorter = MergeSort::new(to_sort_slice);
        let states = sorter.sort();

        assert!(states.len() == 0);
    }

    #[test]
    fn merge_sort_random_test() {
        for _ in 0..10 {
            let mut to_sort_slice = random_vec(30);
            let mut sorter = MergeSort::new(to_sort_slice);
            let states = sorter.sort();

            assert!(states.len() > 0);
            assert!(is_sorted(&states[states.len() - 1]));
        }
    }
}
