use crate::{SortModel, Sorter};

pub struct BubbleSort {}

impl BubbleSort {
    pub fn new() -> Self {
        Self {}
    }

    fn bubble_sort(&self, sorter: &mut SortModel) {
        for i in 0..sorter.len() {
            for j in 0..(sorter.len() - i - 1) {
                if sorter.compare(j, j + 1, |a, b| a > b) {
                    sorter.swap(j, j + 1);
                }
            }
        }
    }
}

impl Sorter for BubbleSort {
    fn sort(&mut self, values: Vec<u32>) -> SortModel {
        let mut sorter = SortModel::new(values);

        self.bubble_sort(&mut sorter);
        sorter.complete();

        sorter
    }
}
