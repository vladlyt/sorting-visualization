use crate::sorting::{SortModel, Sorter};

pub struct MergeSort {}

impl MergeSort {
    pub fn new() -> Self {
        Self {}
    }

    fn merge(&self, sorter: &mut SortModel, mut left: usize, mut mid: usize, right: usize) {
        sorter.set_boundaries(left, right);
        let mut second_left = mid + 1;

        if sorter.compare(mid, second_left, |a, b| a <= b) {
            return;
        }

        while left <= mid && second_left <= right {
            if sorter.compare(left, second_left, |a, b| a <= b) {
                left += 1;
            } else {
                let value = sorter[second_left];
                let mut index = second_left;

                while index != left {
                    sorter.set_value(index, sorter[index - 1]);
                    index -= 1;
                }
                sorter.set_value(left, value);

                left += 1;
                mid += 1;
                second_left += 1;
            }
        }
    }


    fn merge_sort(&self, sorter: &mut SortModel, left: usize, right: usize) {
        if left >= right {
            return;
        }

        let mid = left + (right - left) / 2;
        self.merge_sort(sorter, left, mid);
        self.merge_sort(sorter, mid + 1, right);
        self.merge(sorter, left, mid, right);
    }
}

impl Sorter for MergeSort {
    fn sort(&mut self, values: Vec<u32>) -> SortModel {
        let mut sorter = SortModel::new(values);

        let len = sorter.len();
        if len > 1 {
            self.merge_sort(&mut sorter, 0, len - 1);
        }
        sorter.complete();

        sorter
    }
}

