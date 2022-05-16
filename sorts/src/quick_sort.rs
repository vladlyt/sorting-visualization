use crate::{SortModel, Sorter};

pub struct QuickSort {}

impl QuickSort {
    pub fn new() -> Self {
        Self {}
    }

    pub fn partition(&self, sorter: &mut SortModel, low: usize, high: usize) -> usize {
        let mut i = low;
        sorter.set_boundaries(low, high);

        for j in low..high {
            sorter.set_boundaries(i, high);
            if sorter.compare(j, high, |a, b| a <= b) {
                if i != j {
                    sorter.swap(i, j);
                }
                i += 1;
            }
        }
        sorter.swap(i, high);
        sorter.unset_boundaries();
        i
    }

    fn quick_sort(&self, sorter: &mut SortModel, low: usize, high: usize) {
        if low >= high {
            return;
        }

        let partition_index = self.partition(sorter, low, high);
        if partition_index > 0 {
            self.quick_sort(sorter, low, partition_index - 1);
        }
        self.quick_sort(sorter, partition_index + 1, high);
    }
}

impl Sorter for QuickSort {
    fn sort(&mut self, values: Vec<u32>) -> SortModel {
        let mut sorter = SortModel::new(values);

        let len = sorter.len();
        if len > 1 {
            self.quick_sort(&mut sorter, 0, len - 1);
        }
        sorter.complete();

        sorter
    }
}
