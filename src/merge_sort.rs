use crate::sorting::{SortModel, SortingState, Sorter, SortingValue};

pub struct MergeSort {}

impl MergeSort {
    pub fn new() -> Self {
        Self {}
    }

    fn merge_sort(&self, sorter: &mut SortModel) {
        if sorter.len() <= 1 {
            return;
        }

        let mut left: Vec<u32> = sorter.get_current_state()[0..sorter.len() / 2]
            .iter()
            .map(|v| v.value)
            .collect();
        let mut right: Vec<u32> = sorter.get_current_state()[sorter.len() / 2..sorter.len()]
            .iter()
            .map(|v| v.value)
            .collect();

        let right_backup: SortingState = right.clone().iter().map(|v| SortingValue::new(*v)).collect();

        let mut left_sort = MergeSort::new();
        let mut sorter_left = SortModel::new(left);

        let mut right_sort = MergeSort::new();
        let mut sorter_right = SortModel::new(right);

        left_sort.merge_sort(&mut sorter_left);
        right_sort.merge_sort(&mut sorter_right);
        // TODO rewrite
        sorter.extend_states(
            sorter_left.get_states()
                .iter()
                .map(
                    |state| [&state[..], &right_backup[..]].concat()
                ).collect::<Vec<SortingState>>()
        );
        sorter.extend_states(
            sorter_right.get_states()
                .iter()
                .map(
                    |state| [&sorter_left.get_current_state()[..], &state[..]].concat()
                ).collect::<Vec<SortingState>>()
        );

        let left_len = sorter_left.len();
        let right_len = sorter_right.len();

        sorter.set_current_state([&sorter_left.get_current_state()[..], &sorter_right.get_current_state()[..]].concat());

        let (mut i, mut j, mut k) = (0, 0, 0);
        while i < left_len && j < right_len {
            sorter.compare_index(k);
            if sorter_left.get_current_state()[i] < sorter_right.get_current_state()[j] {
                sorter.set_value(k, sorter_left.get_current_state()[i]);
                i += 1;
            } else {
                sorter.set_value(k, sorter_right.get_current_state()[j]);
                j += 1;
            }
            k += 1;
        }

        while i < left_len {
            // sorter.compare_index(k);
            sorter.set_value(k, sorter_left.get_current_state()[i]);
            i += 1;
            k += 1;
        }

        while j < right_len {
            // sorter.compare_index(k);
            sorter.set_value(k, sorter_right.get_current_state()[j]);
            j += 1;
            k += 1;
        }
    }
}

impl Sorter for MergeSort {
    fn sort(&mut self, values: Vec<u32>) -> SortModel {
        let mut sorter = SortModel::new(values);

        self.merge_sort(&mut sorter);
        sorter.complete();

        sorter
    }
}



#[cfg(test)]
pub mod tests {
    use crate::*;
    use crate::utils::*;

    #[test]
    fn merge_sort_unsorted_test() {
        let mut to_sort_slice = vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1];

        let sorted_model = MergeSort::new().sort(to_sort_slice);

        assert!(is_sorted(sorted_model.get_final_state()));
    }

    #[test]
    fn merge_sort_sorted_test() {
        let mut to_sort_slice = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

        let sorted_model = MergeSort::new().sort(to_sort_slice);

        assert!(is_sorted(sorted_model.get_final_state()));
    }

    #[test]
    fn merge_sort_empty_test() {
        let mut to_sort_slice = vec![];

        let sorted_model = MergeSort::new().sort(to_sort_slice);

        assert_eq!(sorted_model.get_states().len(), 1);
        assert_eq!(sorted_model.get_final_state().len(), 0);
    }

    #[test]
    fn merge_sort_random_test() {
        for _ in 0..10 {
            let mut to_sort_slice = random_vec(30);

            let sorted_model = MergeSort::new().sort(to_sort_slice);

            assert!(is_sorted(sorted_model.get_final_state()));
        }
    }
}
